/*
File Name: analysis.rs
Purpose: Efficient single-pass AST traversal and semantic analysis engine for Thread and Fabric specifications.
*/

#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use miette::Report;

use crate::grammar::thread::Rule as ThreadRule;
use crate::grammar::fabric::Rule as FabricRule;
use crate::helpers::diagnostics::LoomDiagnostic;
use crate::helpers::file_handler::{FileSpanMapping, resolveSpan};

/*
Structure representing a byte location span in source code.
*/
struct SpanInfo
{
    start: usize,
    length: usize,
}

/*
Structure representing an extracted decorator or identifier reference.
*/
struct ReferenceSymbol
{
    target: String,
    span: SpanInfo,
}

fn isDecoratorRule(rule: ThreadRule) -> bool {
    matches!(
        rule,
        ThreadRule::decorator
        | ThreadRule::component_decorator
        | ThreadRule::storage_decorator
        | ThreadRule::protocol_decorator
        | ThreadRule::feature_decorator
        | ThreadRule::diagram_decorator
    )
}

/*
Structure representing an extracted database relation statement to validate.
*/
struct RelationCheckInfo
{
    leftTable: String,
    leftCol: String,
    rightTable: String,
    rightCol: String,
    span: SpanInfo,
    storageName: String,
}

/*
Recursively traverses Pest AST nodes in a single pass to perform semantic validation and collect declarations/references.

Takes:
	pair (pest::iterators::Pair<'_, ThreadRule>): Current Pest AST node pair.
	currentEntity (&mut Option<String>): Reference to active parent entity name.
	currentTable (&mut Option<String>): Reference to active parent table name.
	entityNames (HashMap<String, SpanInfo>): Map of declared entity names to spans.
	memberNames (HashMap<String, SpanInfo>): Map of declared member names (Entity::Member) to spans.
	tableFields (HashMap<(String, String), SpanInfo>): Map of declared table fields (Table, Field) to spans.
	modelFields (HashMap<(String, String), SpanInfo>): Map of declared model fields (Model, Field) to spans.
	tableNames (HashMap<String, SpanInfo>): Map of declared table names to spans.
	diagramNames (HashMap<String, SpanInfo>): Map of declared diagram names to spans.
	references (Vec<ReferenceSymbol>): List of decorator references.
	diagramReferences (HashSet<String>): Set of referenced diagram names.
	relations (Vec<RelationCheckInfo>): List of extracted database relation statements to check.
	protocolChannelTargets (Vec<(String, SpanInfo)>): List of channel target statements (Sender, Receiver, Payload).
	warnings (Vec<Report>): List of accumulated warning reports.
	fileMap (&[FileSpanMapping]): List of file span offset mappings.

Gives:
	Result<(), Report>: Returns Ok(()) on success, or Err(Report) on early exit semantic error.
*/
fn traverseThreadNode(
    pair: pest::iterators::Pair<'_, ThreadRule>,
    currentEntity: &mut Option<String>,
    currentTable: &mut Option<String>,
    entityNames: &mut HashMap<String, SpanInfo>,
    memberNames: &mut HashMap<String, SpanInfo>,
    tableFields: &mut HashMap<(String, String), SpanInfo>,
    modelFields: &mut HashMap<(String, String), SpanInfo>,
    tableNames: &mut HashMap<String, SpanInfo>,
    diagramNames: &mut HashMap<String, SpanInfo>,
    references: &mut Vec<ReferenceSymbol>,
    diagramReferences: &mut HashSet<String>,
    relations: &mut Vec<RelationCheckInfo>,
    protocolChannelTargets: &mut Vec<(String, SpanInfo)>,
    warnings: &mut Vec<Report>,
    fileMap: &[FileSpanMapping],
) -> Result<(), Report>
{
    match pair.as_rule() {
        ThreadRule::feature_entity
        | ThreadRule::component_entity
        | ThreadRule::protocol_entity
        | ThreadRule::storage_entity => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::ident {
                    let eName = child.as_str().to_string();
                    let span = child.as_span();
                    let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

                    /* LM2002: Duplicate Entity Declaration */
                    if entityNames.contains_key(&eName) {
                        let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                        let diag = LoomDiagnostic::new(
                            tPath,
                            tContent.to_string(),
                            lStart,
                            spanInfo.length,
                            format!("Duplicate entity declaration '{}'", eName),
                            Some("Rename the duplicate entity or merge definitions".to_string()),
                            "LM2002".to_string(),
                            format!("Semantic error: Duplicate entity declaration '{}'. Entity was already declared.", eName),
                            miette::Severity::Error,
                        );
                        return Err(diag.toReport());
                    }
                    entityNames.insert(eName.clone(), spanInfo);
                    *currentEntity = Some(eName);
                    break;
                }
            }
        }
        ThreadRule::model_block
        | ThreadRule::contract_block
        | ThreadRule::channel_block
        | ThreadRule::scenario_block
        | ThreadRule::scenario_outline_block => {
            let isScenario = pair.as_rule() == ThreadRule::scenario_block || pair.as_rule() == ThreadRule::scenario_outline_block;
            let mut hasDecorator = false;
            let mut scenarioName = String::from("Scenario");

            for child in pair.clone().into_inner() {
                let r = child.as_rule();
                if r == ThreadRule::ident {
                    let mName = child.as_str().to_string();
                    let span = child.as_span();
                    let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

                    if isScenario {
                        scenarioName = mName.clone();
                    }

                    if let Some(eName) = currentEntity {
                        let scoped = format!("{}::{}", eName, mName);
                        /* LM2002: Duplicate Member Declaration */
                        if memberNames.contains_key(&scoped) {
                            let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                            let diag = LoomDiagnostic::new(
                                tPath,
                                tContent.to_string(),
                                lStart,
                                spanInfo.length,
                                format!("Duplicate member declaration '{}'", mName),
                                Some("Rename the duplicate member or merge definitions".to_string()),
                                "LM2002".to_string(),
                                format!("Semantic error: Duplicate member declaration '{}'. Symbol was already declared.", mName),
                                miette::Severity::Error,
                            );
                            return Err(diag.toReport());
                        }
                        memberNames.insert(scoped, spanInfo);
                    }
                } else if isDecoratorRule(r) {
                    hasDecorator = true;
                }
            }

            /* LM2006: Feature Scenario / Scenario Outline Missing Decorator Verification */
            if isScenario && !hasDecorator {
                let span = pair.as_span();
                let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };
                let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                let diag = LoomDiagnostic::new(
                    tPath,
                    tContent.to_string(),
                    lStart,
                    spanInfo.length,
                    format!("Scenario '{}' missing decorator annotation", scenarioName),
                    Some("Add a @component, @storage, @protocol, or @feature decorator above the scenario".to_string()),
                    "LM2006".to_string(),
                    format!("Semantic warning: Scenario '{}' missing decorator annotation.", scenarioName),
                    miette::Severity::Warning,
                );
                warnings.push(diag.toReport());
            }
        }
        ThreadRule::table_block => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::ident {
                    let tName = child.as_str().to_string();
                    let span = child.as_span();
                    let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

                    *currentTable = Some(tName.clone());
                    tableNames.insert(tName.clone(), SpanInfo { start: spanInfo.start, length: spanInfo.length });

                    if let Some(eName) = currentEntity {
                        let scoped = format!("{}::{}", eName, tName);
                        memberNames.insert(scoped, spanInfo);
                    }
                    break;
                }
            }
        }
        ThreadRule::field_entry => {
            let fStr = pair.as_str();
            let fName = fStr.split(':').next().unwrap_or("").trim().trim_matches('"').to_string();
            let span = pair.as_span();
            let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

            if let Some(tName) = currentTable {
                let scoped = (tName.clone(), fName.clone());
                /* LM2002: Duplicate Field Entry in Table */
                if tableFields.contains_key(&scoped) {
                    let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                    let diag = LoomDiagnostic::new(
                        tPath,
                        tContent.to_string(),
                        lStart,
                        spanInfo.length,
                        format!("Duplicate field declaration '{}'", fName),
                        Some("Rename the duplicate field or remove redundant declaration".to_string()),
                        "LM2002".to_string(),
                        format!("Semantic error: Duplicate field declaration '{}'. Field was already declared in table.", fName),
                        miette::Severity::Error,
                    );
                    return Err(diag.toReport());
                }
                tableFields.insert(scoped, spanInfo);
            }
        }
        ThreadRule::model_member => {
            let mStr = pair.as_str();
            let mName = mStr.split(':').next().unwrap_or("").trim().trim_matches('"').to_string();
            let span = pair.as_span();
            let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

            if let Some(eName) = currentEntity {
                let scoped = (eName.clone(), mName.clone());
                /* LM2002: Duplicate Member Field in Model */
                if modelFields.contains_key(&scoped) {
                    let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                    let diag = LoomDiagnostic::new(
                        tPath,
                        tContent.to_string(),
                        lStart,
                        spanInfo.length,
                        format!("Duplicate model member '{}'", mName),
                        Some("Rename the duplicate model member or remove redundant declaration".to_string()),
                        "LM2002".to_string(),
                        format!("Semantic error: Duplicate model member '{}'. Member was already declared in model.", mName),
                        miette::Severity::Error,
                    );
                    return Err(diag.toReport());
                }
                modelFields.insert(scoped, spanInfo);
            }
        }
        ThreadRule::index_decl => {
            let idxStr = pair.as_str();
            let span = pair.as_span();
            let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

            if let (Some(open), Some(close)) = (idxStr.find('('), idxStr.find(')')) {
                let colName = idxStr[open + 1..close].trim().to_string();
                if let Some(tName) = currentTable {
                    /* LM2003: Storage Index Column Verification against tableFields AST */
                    if !tableFields.contains_key(&(tName.clone(), colName.clone())) {
                        let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
                        let diag = LoomDiagnostic::new(
                            tPath,
                            tContent.to_string(),
                            lStart,
                            spanInfo.length,
                            format!("Storage index column '{}' not found in table '{}'", colName, tName),
                            Some(format!("Ensure index column '{}' is defined as a field in table '{}'", colName, tName)),
                            "LM2003".to_string(),
                            format!("Semantic error: Storage index column '{}' not found in table '{}'.", colName, tName),
                            miette::Severity::Error,
                        );
                        return Err(diag.toReport());
                    }
                }
            }
        }
        ThreadRule::db_relation => {
            let span = pair.as_span();
            let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

            let mut colRefs = Vec::new();
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::column_ref {
                    let rawRef = child.as_str();
                    let cleanRef = if let Some(colIdx) = rawRef.find("::") { &rawRef[colIdx + 2..] } else { rawRef };
                    if let Some(dotIdx) = cleanRef.find('.') {
                        let relTable = cleanRef[..dotIdx].trim().to_string();
                        let relCol = cleanRef[dotIdx + 1..].trim().to_string();
                        colRefs.push((relTable, relCol));
                    }
                }
            }

            if colRefs.len() == 2 {
                relations.push(RelationCheckInfo {
                    leftTable: colRefs[0].0.clone(),
                    leftCol: colRefs[0].1.clone(),
                    rightTable: colRefs[1].0.clone(),
                    rightCol: colRefs[1].1.clone(),
                    span: spanInfo,
                    storageName: currentEntity.as_deref().unwrap_or("AppStorage").to_string(),
                });
            }
        }
        ThreadRule::sender_stmt | ThreadRule::receiver_stmt | ThreadRule::payload_stmt => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::string_lit {
                    let targetVal = child.as_str().trim_matches('"').to_string();
                    let span = child.as_span();
                    let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };
                    protocolChannelTargets.push((targetVal, spanInfo));
                }
            }
        }
        ThreadRule::component_decorator
        | ThreadRule::storage_decorator
        | ThreadRule::protocol_decorator
        | ThreadRule::feature_decorator
        | ThreadRule::diagram_decorator
        | ThreadRule::decorator => {
            let decStr = pair.as_str();
            let span = pair.as_span();
            let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };

            if let (Some(open), Some(close)) = (decStr.find('('), decStr.find(')')) {
                let target = decStr[open + 1..close].trim().to_string();
                references.push(ReferenceSymbol { target: target.clone(), span: spanInfo });
                if decStr.starts_with("@diagram") {
                    diagramReferences.insert(target);
                }
            }
        }
        ThreadRule::diagram_block => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::ident {
                    let dName = child.as_str().to_string();
                    let span = child.as_span();
                    let spanInfo = SpanInfo { start: span.start(), length: span.end() - span.start() };
                    diagramNames.insert(dName, spanInfo);
                    break;
                }
            }
        }
        _ => {}
    }

    /* Recursively walk inner Pest AST child nodes */
    for child in pair.into_inner() {
        traverseThreadNode(
            child,
            currentEntity,
            currentTable,
            entityNames,
            memberNames,
            tableFields,
            modelFields,
            tableNames,
            diagramNames,
            references,
            diagramReferences,
            relations,
            protocolChannelTargets,
            warnings,
            fileMap,
        )?;
    }

    Ok(())
}

/*
Performs semantic analysis over parsed thread specification AST pairs by reference, returning warnings or errors.

Takes:
	pairs (&pest::iterators::Pairs<'_, ThreadRule>): The root Pest AST pairs reference for a Thread file.
	fileMap (&[FileSpanMapping]): List of ingested file span offset mappings.

Gives:
	Result<Vec<Report>, Report>: Returns list of warnings if no errors, or an error report on semantic error.
*/
pub fn checkThread(
    pairs: &pest::iterators::Pairs<'_, ThreadRule>,
    fileMap: &[FileSpanMapping],
) -> Result<Vec<Report>, Report>
{
    let mut entityNames: HashMap<String, SpanInfo> = HashMap::new();
    let mut memberNames: HashMap<String, SpanInfo> = HashMap::new();
    let mut tableFields: HashMap<(String, String), SpanInfo> = HashMap::new();
    let mut modelFields: HashMap<(String, String), SpanInfo> = HashMap::new();
    let mut tableNames: HashMap<String, SpanInfo> = HashMap::new();
    let mut diagramNames: HashMap<String, SpanInfo> = HashMap::new();

    let mut references: Vec<ReferenceSymbol> = Vec::new();
    let mut diagramReferences: HashSet<String> = HashSet::new();
    let mut relations: Vec<RelationCheckInfo> = Vec::new();
    let mut protocolChannelTargets: Vec<(String, SpanInfo)> = Vec::new();
    let mut warnings: Vec<Report> = Vec::new();

    let mut currentEntity: Option<String> = None;
    let mut currentTable: Option<String> = None;

    /* Single-pass AST Traversal across all root nodes */
    for pair in pairs.clone() {
        traverseThreadNode(
            pair,
            &mut currentEntity,
            &mut currentTable,
            &mut entityNames,
            &mut memberNames,
            &mut tableFields,
            &mut modelFields,
            &mut tableNames,
            &mut diagramNames,
            &mut references,
            &mut diagramReferences,
            &mut relations,
            &mut protocolChannelTargets,
            &mut warnings,
            fileMap,
        )?;
    }

    /* LM2004: Storage Relation Target Verification against AST tableNames & tableFields */
    for relInfo in &relations {
        let leftTableOk = tableNames.contains_key(&relInfo.leftTable);
        let leftColOk = tableFields.contains_key(&(relInfo.leftTable.clone(), relInfo.leftCol.clone()));
        let rightTableOk = tableNames.contains_key(&relInfo.rightTable);
        let rightColOk = tableFields.contains_key(&(relInfo.rightTable.clone(), relInfo.rightCol.clone()));

        if !leftTableOk || !leftColOk {
            let invalidRef = format!("{}.{}", relInfo.leftTable, relInfo.leftCol);
            let (tPath, tContent, lStart) = resolveSpan(fileMap, relInfo.span.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                relInfo.span.length,
                format!("Storage relation target '{}' does not exist in Storage '{}'", invalidRef, relInfo.storageName),
                Some(format!("Ensure target table and field exist in Storage '{}'", relInfo.storageName)),
                "LM2004".to_string(),
                format!("Semantic error: Storage relation target '{}' does not exist in Storage '{}'.", invalidRef, relInfo.storageName),
                miette::Severity::Error,
            );
            return Err(diag.toReport());
        }

        if !rightTableOk || !rightColOk {
            let invalidRef = format!("{}.{}", relInfo.rightTable, relInfo.rightCol);
            let (tPath, tContent, lStart) = resolveSpan(fileMap, relInfo.span.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                relInfo.span.length,
                format!("Storage relation target '{}' does not exist in Storage '{}'", invalidRef, relInfo.storageName),
                Some(format!("Ensure target table and field exist in Storage '{}'", relInfo.storageName)),
                "LM2004".to_string(),
                format!("Semantic error: Storage relation target '{}' does not exist in Storage '{}'.", invalidRef, relInfo.storageName),
                miette::Severity::Error,
            );
            return Err(diag.toReport());
        }
    }

    /* LM2005: Protocol Channel Target Verification against declared AST entities & members */
    for (targetVal, spanInfo) in &protocolChannelTargets {
        let containsField = targetVal.contains('.');
        let isPrimitiveType = ["String", "UUID", "Int", "Boolean", "Float", "DateTime", "Text"].contains(&targetVal.as_str());

        let targetValid = if containsField {
            false
        } else if isPrimitiveType {
            true
        } else if targetVal.contains("::") {
            /* Scoped target Entity::Member must exist in declared memberNames */
            memberNames.contains_key(targetVal)
        } else {
            /* Unscoped target Entity must exist in declared entityNames */
            entityNames.contains_key(targetVal)
        };

        if !targetValid {
            let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                spanInfo.length,
                format!("Channel target '{}' does not match any known entity or member", targetVal),
                Some("Ensure Sender, Receiver, and Payload point to valid declared Entity or Member names (fields are not permitted)".to_string()),
                "LM2005".to_string(),
                format!("Semantic error: Channel target '{}' does not match any known entity or member. Only Entity and Member names are allowed (fields are not permitted).", targetVal),
                miette::Severity::Error,
            );
            return Err(diag.toReport());
        }
    }

    /* LM2001: Check unresolved decorator references against extracted entity/member/diagram AST symbols */
    for refSym in &references {
        if !entityNames.contains_key(&refSym.target) && !memberNames.contains_key(&refSym.target) && !diagramNames.contains_key(&refSym.target) {
            let (tPath, tContent, lStart) = resolveSpan(fileMap, refSym.span.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                refSym.span.length,
                format!("Unresolved reference '{}'", refSym.target),
                Some("Define missing entity or member, or fix reference path in decorator".to_string()),
                "LM2001".to_string(),
                format!("Semantic error: Unresolved reference '{}'. Target entity or member was not found.", refSym.target),
                miette::Severity::Error,
            );
            return Err(diag.toReport());
        }
    }

    /* Warnings Checks: Unified LM2007 Unused Symbol Verification */
    let mut refTargetsSet: HashSet<String> = references.iter().map(|r| r.target.clone()).collect();
    for (chTarget, _) in &protocolChannelTargets {
        refTargetsSet.insert(chTarget.clone());
    }

    /* LM2007: Entirely Unused Entity Verification */
    for (entity, spanInfo) in &entityNames {
        let isReferenced = refTargetsSet.contains(entity) || refTargetsSet.iter().any(|r| r.starts_with(&format!("{}::", entity)));
        if !isReferenced {
            let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                spanInfo.length,
                format!("Unused entity '{}'", entity),
                Some("Reference entity in relevant feature scenarios or remove unused declaration".to_string()),
                "LM2007".to_string(),
                format!("Semantic warning: Unused entity '{}'. Entity is declared but never referenced.", entity),
                miette::Severity::Warning,
            );
            warnings.push(diag.toReport());
        }
    }

    /* LM2007: Partially Unused Entity Member Verification */
    for (member, spanInfo) in &memberNames {
        if !refTargetsSet.contains(member) {
            let parts: Vec<&str> = member.split("::").collect();
            let mName = if parts.len() > 1 { parts[1] } else { member.as_str() };
            let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                spanInfo.length,
                format!("Unused member '{}'", mName),
                Some("Reference member in relevant feature scenarios or remove unused declaration".to_string()),
                "LM2007".to_string(),
                format!("Semantic warning: Unused member '{}'. Member is declared but never referenced.", mName),
                miette::Severity::Warning,
            );
            warnings.push(diag.toReport());
        }
    }

    /* LM2007: Unused Local !Diagram Verification */
    for (diagName, spanInfo) in &diagramNames {
        if !diagramReferences.contains(diagName) {
            let (tPath, tContent, lStart) = resolveSpan(fileMap, spanInfo.start);
            let diag = LoomDiagnostic::new(
                tPath,
                tContent.to_string(),
                lStart,
                spanInfo.length,
                format!("Unused diagram '{}'", diagName),
                Some("Reference diagram in relevant feature scenarios or remove unused declaration".to_string()),
                "LM2007".to_string(),
                format!("Semantic warning: Unused diagram '{}'. Diagram is declared but never referenced.", diagName),
                miette::Severity::Warning,
            );
            warnings.push(diag.toReport());
        }
    }

    Ok(warnings)
}

fn collectThreadSymbols(
    pair: pest::iterators::Pair<'_, ThreadRule>,
    currentEntity: &mut Option<String>,
    threadEntities: &mut HashSet<String>,
    threadMembers: &mut HashSet<String>,
)
{
    match pair.as_rule() {
        ThreadRule::feature_entity
        | ThreadRule::component_entity
        | ThreadRule::protocol_entity
        | ThreadRule::storage_entity => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::ident {
                    let eName = child.as_str().to_string();
                    threadEntities.insert(eName.clone());
                    *currentEntity = Some(eName);
                    break;
                }
            }
        }
        ThreadRule::model_block
        | ThreadRule::contract_block
        | ThreadRule::table_block
        | ThreadRule::channel_block
        | ThreadRule::scenario_block
        | ThreadRule::scenario_outline_block => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == ThreadRule::ident {
                    let mName = child.as_str().to_string();
                    if let Some(eName) = currentEntity {
                        threadMembers.insert(format!("{}::{}", eName, mName));
                    }
                    break;
                }
            }
        }
        _ => {}
    }

    for child in pair.into_inner() {
        collectThreadSymbols(child, currentEntity, threadEntities, threadMembers);
    }
}

fn collectFabricReferences<'a>(
    pair: pest::iterators::Pair<'a, FabricRule>,
    fabricReferencedTargets: &mut HashSet<String>,
    fabricRefSpans: &mut Vec<(String, pest::Span<'a>)>,
)
{
    if pair.as_rule() == FabricRule::entity_ref {
        let span = pair.as_span();
        let mut path = String::new();
        for rChild in pair.clone().into_inner() {
            if rChild.as_rule() == FabricRule::scoped_path {
                path = rChild.as_str().to_string();
            }
        }
        if !path.is_empty() {
            fabricReferencedTargets.insert(path.clone());
            fabricRefSpans.push((path, span));
        }
    }

    for child in pair.into_inner() {
        collectFabricReferences(child, fabricReferencedTargets, fabricRefSpans);
    }
}

/*
Performs cross-referencing semantic analysis over parsed fabric blueprint AST pairs and thread specification AST pairs by reference.

Takes:
	threadPairs (&pest::iterators::Pairs<'_, ThreadRule>): The root Pest AST pairs reference for ingested Thread specifications.
	fabricPairs (&pest::iterators::Pairs<'_, FabricRule>): The root Pest AST pairs reference for a Fabric blueprint.
	fabricFileMap (&[FileSpanMapping]): List of fabric file span offset mappings.

Gives:
	Result<Vec<Report>, Report>: Returns list of warnings if no errors, or an error report on semantic error.
*/
pub fn checkFabric(
    threadPairs: &pest::iterators::Pairs<'_, ThreadRule>,
    fabricPairs: &pest::iterators::Pairs<'_, FabricRule>,
    fabricFileMap: &[FileSpanMapping],
) -> Result<Vec<Report>, Report>
{
    let mut threadEntities: HashSet<String> = HashSet::new();
    let mut threadMembers: HashSet<String> = HashSet::new();
    let mut currentEntity: Option<String> = None;

    /* Step 1: Recursively extract declared thread entities and members from thread AST pairs */
    for pair in threadPairs.clone() {
        collectThreadSymbols(pair, &mut currentEntity, &mut threadEntities, &mut threadMembers);
    }

    /* Step 2: Recursively extract fabric references */
    let mut fabricReferencedTargets: HashSet<String> = HashSet::new();
    let mut fabricRefSpans = Vec::new();
    for pair in fabricPairs.clone() {
        collectFabricReferences(pair, &mut fabricReferencedTargets, &mut fabricRefSpans);
    }

    /* Step 3: Check LM3001 errors for unresolved fabric references */
    for (path, span) in &fabricRefSpans {
        let existsInThread = threadEntities.contains(path) || threadMembers.contains(path);
        if !existsInThread {
            let (targetPath, targetContent, localStart) = resolveSpan(fabricFileMap, span.start());
            let diag = LoomDiagnostic::new(
                targetPath,
                targetContent.to_string(),
                localStart,
                span.end() - span.start(),
                format!("Unresolved fabric reference '{}'", path),
                Some("Define missing thread entity or correct reference in fabric blueprint".to_string()),
                "LM3001".to_string(),
                format!("Semantic error: Unresolved fabric reference '{}'. Target thread entity was not found.", path),
                miette::Severity::Error,
            );
            return Err(diag.toReport());
        }
    }

    /* Step 4: Check LM3002 warnings for declared thread entities or members not referenced in fabric */
    let mut warnings: Vec<Report> = Vec::new();
    for entity in &threadEntities {
        let isRef = fabricReferencedTargets.contains(entity) || fabricReferencedTargets.iter().any(|r| r.starts_with(&format!("{}::", entity)));
        if !isRef {
            let (targetPath, targetContent, _) = resolveSpan(fabricFileMap, 0);
            let diag = LoomDiagnostic::new(
                targetPath,
                targetContent.to_string(),
                0,
                targetContent.len(),
                format!("Unused thread entity or member '{}' in fabric blueprint", entity),
                Some("Reference entity or member in fabric blueprint connections or remove unused declaration".to_string()),
                "LM3002".to_string(),
                format!("Semantic warning: Thread entity or member '{}' is declared but not referenced in fabric blueprint.", entity),
                miette::Severity::Warning,
            );
            warnings.push(diag.toReport());
        }
    }

    Ok(warnings)
}
