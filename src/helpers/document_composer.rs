/*
File Name: document_composer.rs
Purpose: AST document composition engine converting Thread and Fabric Pest AST pairs into comprehensive, highly structured JSON representations for frontend rendering.
*/

#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

use crate::grammar::thread::Rule as ThreadRule;
use crate::grammar::fabric::Rule as FabricRule;

/*
Structure representing a step statement in a scenario.
*/
#[derive(Serialize, Deserialize)]
struct StepDto
{
    keyword: String,
    text: String,
}

/*
Structure representing a decorator attached to an entity, contract, model, table, channel, or scenario.
*/
#[derive(Serialize, Deserialize)]
struct DecoratorDto
{
    decoratorType: String,
    target: String,
}

/*
Structure representing a local diagram in an entity.
*/
#[derive(Serialize, Deserialize)]
struct DiagramDto
{
    name: String,
    content: String,
}

/*
Structure representing a scenario or scenario outline in a feature.
*/
#[derive(Serialize, Deserialize)]
struct ScenarioDto
{
    name: String,
    isOutline: bool,
    decorators: Vec<DecoratorDto>,
    steps: Vec<StepDto>,
    examples: Vec<String>,
}

/*
Structure representing a rule container in a feature.
*/
#[derive(Serialize, Deserialize)]
struct RuleDto
{
    name: String,
    scenarios: Vec<ScenarioDto>,
}

/*
Structure representing a feature specification entity.
*/
#[derive(Serialize, Deserialize)]
struct FeatureDto
{
    name: String,
    notes: Vec<String>,
    diagrams: Vec<DiagramDto>,
    backgroundSteps: Vec<StepDto>,
    rules: Vec<RuleDto>,
    scenarios: Vec<ScenarioDto>,
}

/*
Structure representing a contract specification in a component.
*/
#[derive(Serialize, Deserialize)]
struct ContractDto
{
    name: String,
    decorators: Vec<DecoratorDto>,
    signature: String,
    requires: String,
    ensures: String,
    preconditions: Vec<String>,
    postconditions: Vec<String>,
    process: Vec<String>,
    errors: Vec<String>,
}

/*
Structure representing a model specification in a component.
*/
#[derive(Serialize, Deserialize)]
struct ModelDto
{
    name: String,
    decorators: Vec<DecoratorDto>,
    modelType: String,
    members: Vec<Vec<String>>,
}

/*
Structure representing a component specification entity.
*/
#[derive(Serialize, Deserialize)]
struct ComponentDto
{
    name: String,
    notes: Vec<String>,
    diagrams: Vec<DiagramDto>,
    invariants: Vec<String>,
    contracts: Vec<ContractDto>,
    models: Vec<ModelDto>,
}

/*
Structure representing a database table field.
*/
#[derive(Serialize, Deserialize)]
struct TableFieldDto
{
    name: String,
    fieldType: String,
}

/*
Structure representing a database relation statement.
*/
#[derive(Serialize, Deserialize)]
struct RelationDto
{
    leftTable: String,
    leftColumn: String,
    relationType: String,
    rightTable: String,
    rightColumn: String,
}

/*
Structure representing a database table in a storage entity.
*/
#[derive(Serialize, Deserialize)]
struct TableDto
{
    name: String,
    decorators: Vec<DecoratorDto>,
    fields: Vec<TableFieldDto>,
    indexes: Vec<String>,
    relations: Vec<RelationDto>,
}

/*
Structure representing a storage specification entity.
*/
#[derive(Serialize, Deserialize)]
struct StorageDto
{
    name: String,
    engine: String,
    notes: Vec<String>,
    diagrams: Vec<DiagramDto>,
    tables: Vec<TableDto>,
}

/*
Structure representing a protocol channel statement.
*/
#[derive(Serialize, Deserialize)]
struct ChannelDto
{
    name: String,
    decorators: Vec<DecoratorDto>,
    pattern: String,
    transport: String,
    sender: String,
    receiver: String,
    payload: String,
    errors: Vec<String>,
}

/*
Structure representing a protocol specification entity.
*/
#[derive(Serialize, Deserialize)]
struct ProtocolDto
{
    name: String,
    notes: Vec<String>,
    diagrams: Vec<DiagramDto>,
    channels: Vec<ChannelDto>,
}

/*
Structure representing the complete Thread specification document JSON payload.
*/
#[derive(Serialize, Deserialize)]
struct ThreadDocumentDto
{
    features: Vec<FeatureDto>,
    components: Vec<ComponentDto>,
    storages: Vec<StorageDto>,
    protocols: Vec<ProtocolDto>,
}

/*
Structure representing an entity reference in fabric.
*/
#[derive(Serialize, Deserialize, Clone)]
struct FabricEntityRefDto
{
    kind: String,
    path: String,
}

/*
Structure representing a cluster group in fabric.
*/
#[derive(Serialize, Deserialize)]
struct FabricGroupDto
{
    name: String,
    members: Vec<FabricEntityRefDto>,
}

/*
Structure representing a connection edge in fabric.
*/
#[derive(Serialize, Deserialize)]
struct FabricConnectionDto
{
    source: FabricEntityRefDto,
    target: FabricEntityRefDto,
    label: String,
}

/*
Structure representing the complete Fabric blueprint JSON payload.
*/
#[derive(Serialize, Deserialize)]
struct FabricDocumentDto
{
    system: String,
    groups: Vec<FabricGroupDto>,
    connections: Vec<FabricConnectionDto>,
}

/*
Composes a Thread Pest AST Pairs structure into a formatted JSON string by reference containing 95%+ extracted specification data.

Takes:
	pairs (&pest::iterators::Pairs<'_, ThreadRule>): The root Pest AST pairs reference for a Thread file.

Gives:
	Result<String, String>: Returns pretty JSON string on success, or error string on failure.
*/
pub fn composeThreadAst(
    pairs: &pest::iterators::Pairs<'_, ThreadRule>,
) -> Result<String, String>
{
    let mut doc = ThreadDocumentDto {
        features: Vec::new(),
        components: Vec::new(),
        storages: Vec::new(),
        protocols: Vec::new(),
    };

    for pair in pairs.clone() {
        traverseThreadComposerNode(&pair, &mut doc);
    }

    serde_json::to_string_pretty(&doc).map_err(|err| format!("Failed to serialize thread document to JSON: {}", err))
}

/*
Composes a Fabric Pest AST Pairs structure into a formatted JSON string by reference containing 95%+ extracted blueprint data.

Takes:
	pairs (&pest::iterators::Pairs<'_, FabricRule>): The root Pest AST pairs reference for a Fabric file.

Gives:
	Result<String, String>: Returns pretty JSON string on success, or error string on failure.
*/
pub fn composeFabricAst(
    pairs: &pest::iterators::Pairs<'_, FabricRule>,
) -> Result<String, String>
{
    let mut doc = FabricDocumentDto {
        system: String::from("Loom System Architecture"),
        groups: Vec::new(),
        connections: Vec::new(),
    };

    for pair in pairs.clone() {
        traverseFabricComposerNode(&pair, &mut doc);
    }

    serde_json::to_string_pretty(&doc).map_err(|err| format!("Failed to serialize fabric document to JSON: {}", err))
}

fn traverseThreadComposerNode(
    pair: &pest::iterators::Pair<'_, ThreadRule>,
    doc: &mut ThreadDocumentDto,
)
{
    match pair.as_rule() {
        ThreadRule::feature_entity => {
            let feat = extractFeatureEntity(pair);
            doc.features.push(feat);
            return;
        }
        ThreadRule::component_entity => {
            let comp = extractComponentEntity(pair);
            doc.components.push(comp);
            return;
        }
        ThreadRule::storage_entity => {
            let stor = extractStorageEntity(pair);
            doc.storages.push(stor);
            return;
        }
        ThreadRule::protocol_entity => {
            let prot = extractProtocolEntity(pair);
            doc.protocols.push(prot);
            return;
        }
        _ => {}
    }

    for child in pair.clone().into_inner() {
        traverseThreadComposerNode(&child, doc);
    }
}

fn traverseFabricComposerNode(
    pair: &pest::iterators::Pair<'_, FabricRule>,
    doc: &mut FabricDocumentDto,
)
{
    match pair.as_rule() {
        FabricRule::system_decl => {
            for child in pair.clone().into_inner() {
                if child.as_rule() == FabricRule::string_lit {
                    doc.system = child.as_str().trim_matches('"').to_string();
                }
            }
        }
        FabricRule::group_block => {
            let group = extractFabricGroup(pair);
            doc.groups.push(group);
            return;
        }
        FabricRule::connection_stmt => {
            let conn = extractFabricConnection(pair);
            doc.connections.push(conn);
            return;
        }
        _ => {}
    }

    for child in pair.clone().into_inner() {
        traverseFabricComposerNode(&child, doc);
    }
}

fn extractNote(pair: &pest::iterators::Pair<'_, ThreadRule>) -> String
{
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::note_content {
            return child.as_str().trim().to_string();
        }
    }
    pair.as_str().replace("!Note", "").trim().trim_start_matches('[').trim_end_matches(']').trim().to_string()
}

fn extractDiagram(pair: &pest::iterators::Pair<'_, ThreadRule>) -> DiagramDto
{
    let mut dName = String::new();
    let mut dContent = String::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::ident {
            dName = child.as_str().to_string();
        } else if child.as_rule() == ThreadRule::diagram_content {
            dContent = child.as_str().trim().to_string();
        }
    }
    DiagramDto { name: dName, content: dContent }
}

fn extractDecorator(pair: &pest::iterators::Pair<'_, ThreadRule>) -> DecoratorDto
{
    let decStr = pair.as_str();
    let dType = decStr.split('(').next().unwrap_or("").trim_start_matches('@').to_string();
    let target = if let (Some(open), Some(close)) = (decStr.find('('), decStr.find(')')) {
        decStr[open + 1..close].trim().to_string()
    } else {
        String::new()
    };
    DecoratorDto { decoratorType: dType, target }
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

fn extractFeatureEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> FeatureDto
{
    let mut fName = String::new();
    let mut notes = Vec::new();
    let mut diagrams = Vec::new();
    let mut backgroundSteps = Vec::new();
    let mut rules = Vec::new();
    let mut scenarios = Vec::new();

    let mut nodesToProcess = Vec::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::feature_body_item {
            nodesToProcess.extend(child.into_inner());
        } else {
            nodesToProcess.push(child);
        }
    }

    for child in nodesToProcess {
        match child.as_rule() {
            ThreadRule::ident => {
                if fName.is_empty() {
                    fName = child.as_str().to_string();
                }
            }
            ThreadRule::note_block => {
                notes.push(extractNote(&child));
            }
            ThreadRule::diagram_block => {
                diagrams.push(extractDiagram(&child));
            }
            ThreadRule::background_block => {
                for bChild in child.into_inner() {
                    if bChild.as_rule() == ThreadRule::step_stmt {
                        backgroundSteps.push(extractStep(&bChild));
                    }
                }
            }
            ThreadRule::rule_block => {
                let mut rName = String::new();
                let mut rScenarios = Vec::new();
                for rChild in child.into_inner() {
                    if rChild.as_rule() == ThreadRule::string_lit {
                        rName = rChild.as_str().trim_matches('"').to_string();
                    } else if rChild.as_rule() == ThreadRule::scenario_block || rChild.as_rule() == ThreadRule::scenario_outline_block {
                        rScenarios.push(extractScenario(&rChild));
                    }
                }
                rules.push(RuleDto { name: rName, scenarios: rScenarios });
            }
            ThreadRule::scenario_block | ThreadRule::scenario_outline_block => {
                scenarios.push(extractScenario(&child));
            }
            _ => {}
        }
    }

    FeatureDto { name: fName, notes, diagrams, backgroundSteps, rules, scenarios }
}

fn extractScenario(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ScenarioDto
{
    let isOutline = pair.as_rule() == ThreadRule::scenario_outline_block;
    let mut sName = String::new();
    let mut decorators = Vec::new();
    let mut steps = Vec::new();
    let mut examples = Vec::new();

    for child in pair.clone().into_inner() {
        if isDecoratorRule(child.as_rule()) {
            decorators.push(extractDecorator(&child));
        } else {
            match child.as_rule() {
                ThreadRule::string_lit => {
                    if sName.is_empty() {
                        sName = child.as_str().trim_matches('"').to_string();
                    }
                }
                ThreadRule::step_stmt => {
                    steps.push(extractStep(&child));
                }
                ThreadRule::examples_block => {
                    for exChild in child.into_inner() {
                        if exChild.as_rule() == ThreadRule::examples_row {
                            examples.push(exChild.as_str().trim().to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ScenarioDto { name: sName, isOutline, decorators, steps, examples }
}

fn extractStep(pair: &pest::iterators::Pair<'_, ThreadRule>) -> StepDto
{
    let mut kw = String::from("Given");
    let mut txt = String::new();

    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::step_kw {
            kw = child.as_str().to_string();
        } else if child.as_rule() == ThreadRule::string_lit {
            txt = child.as_str().trim_matches('"').to_string();
        }
    }

    StepDto { keyword: kw, text: txt }
}

fn extractComponentEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ComponentDto
{
    let mut cName = String::new();
    let mut notes = Vec::new();
    let mut diagrams = Vec::new();
    let mut invariants = Vec::new();
    let mut contracts = Vec::new();
    let mut models = Vec::new();

    let mut nodesToProcess = Vec::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::component_body_item {
            nodesToProcess.extend(child.into_inner());
        } else {
            nodesToProcess.push(child);
        }
    }

    for child in nodesToProcess {
        match child.as_rule() {
            ThreadRule::ident => {
                if cName.is_empty() {
                    cName = child.as_str().to_string();
                }
            }
            ThreadRule::note_block => {
                notes.push(extractNote(&child));
            }
            ThreadRule::diagram_block => {
                diagrams.push(extractDiagram(&child));
            }
            ThreadRule::invariants_block => {
                for invChild in child.into_inner() {
                    if invChild.as_rule() == ThreadRule::string_lit {
                        invariants.push(invChild.as_str().trim_matches('"').to_string());
                    }
                }
            }
            ThreadRule::contract_block => {
                contracts.push(extractContractEntity(&child));
            }
            ThreadRule::model_block => {
                models.push(extractModelEntity(&child));
            }
            _ => {}
        }
    }

    ComponentDto { name: cName, notes, diagrams, invariants, contracts, models }
}

fn extractContractEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ContractDto
{
    let mut ctName = String::new();
    let mut decorators = Vec::new();
    let mut sig = String::new();
    let mut req = String::new();
    let mut ens = String::new();
    let mut pre = Vec::new();
    let mut post = Vec::new();
    let mut proc = Vec::new();
    let mut errs = Vec::new();

    let mut ctItems = Vec::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::contract_body_item {
            ctItems.extend(child.into_inner());
        } else {
            ctItems.push(child);
        }
    }

    for child in ctItems {
        if isDecoratorRule(child.as_rule()) {
            decorators.push(extractDecorator(&child));
        } else {
            match child.as_rule() {
                ThreadRule::ident => {
                    if ctName.is_empty() {
                        ctName = child.as_str().to_string();
                    }
                }
                ThreadRule::signature_stmt => {
                    sig = child.as_str().replace("Signature", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::requires_stmt => {
                    req = child.as_str().replace("Requires", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::ensures_stmt => {
                    ens = child.as_str().replace("Ensures", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::precondition_block => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            pre.push(stmt.as_str().trim_matches('"').to_string());
                        }
                    }
                }
                ThreadRule::postcondition_block => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            post.push(stmt.as_str().trim_matches('"').to_string());
                        }
                    }
                }
                ThreadRule::process_block => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::process_step {
                            proc.push(stmt.as_str().trim().to_string());
                        }
                    }
                }
                ThreadRule::errors_block => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            errs.push(stmt.as_str().trim_matches('"').to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ContractDto {
        name: ctName,
        decorators,
        signature: sig,
        requires: req,
        ensures: ens,
        preconditions: pre,
        postconditions: post,
        process: proc,
        errors: errs,
    }
}

fn extractModelEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ModelDto
{
    let mut mName = String::new();
    let mut decorators = Vec::new();
    let mut mType = String::from("Struct");
    let mut members: Vec<Vec<String>> = Vec::new();

    for child in pair.clone().into_inner() {
        if isDecoratorRule(child.as_rule()) {
            decorators.push(extractDecorator(&child));
        } else {
            match child.as_rule() {
                ThreadRule::ident => {
                    if mName.is_empty() {
                        mName = child.as_str().to_string();
                    }
                }
                ThreadRule::model_type_stmt => {
                    mType = child.as_str().replace("Type", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::members_block => {
                    for memb in child.into_inner() {
                        if memb.as_rule() == ThreadRule::model_member {
                            let raw = memb.as_str();
                            let mut parts = raw.split(':');
                            let fnm = parts.next().unwrap_or("").trim().trim_matches('"').to_string();
                            let ftp = parts.next().unwrap_or("").trim().trim_matches('"').to_string();
                            if ftp.is_empty() {
                                members.push(vec![fnm]);
                            } else {
                                members.push(vec![fnm, ftp]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ModelDto { name: mName, decorators, modelType: mType, members }
}

fn extractStorageEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> StorageDto
{
    let mut sName = String::new();
    let mut engine = String::new();
    let mut notes = Vec::new();
    let mut diagrams = Vec::new();
    let mut tables = Vec::new();

    for child in pair.clone().into_inner() {
        match child.as_rule() {
            ThreadRule::ident => {
                if sName.is_empty() {
                    sName = child.as_str().to_string();
                }
            }
            ThreadRule::engine_stmt => {
                engine = child.as_str().replace("Engine", "").trim().trim_matches('"').to_string();
            }
            ThreadRule::note_block => {
                notes.push(extractNote(&child));
            }
            ThreadRule::diagram_block => {
                diagrams.push(extractDiagram(&child));
            }
            ThreadRule::table_block => {
                tables.push(extractTableEntity(&child));
            }
            _ => {}
        }
    }

    StorageDto { name: sName, engine, notes, diagrams, tables }
}

fn extractTableEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> TableDto
{
    let mut tName = String::new();
    let mut decorators = Vec::new();
    let mut fields = Vec::new();
    let mut indexes = Vec::new();
    let mut relations = Vec::new();

    let mut tItems = Vec::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::table_body_item {
            tItems.extend(child.into_inner());
        } else {
            tItems.push(child);
        }
    }

    for child in tItems {
        if isDecoratorRule(child.as_rule()) {
            decorators.push(extractDecorator(&child));
        } else {
            match child.as_rule() {
                ThreadRule::ident => {
                    if tName.is_empty() {
                        tName = child.as_str().to_string();
                    }
                }
                ThreadRule::fields_block => {
                    for fEntry in child.into_inner() {
                        if fEntry.as_rule() == ThreadRule::field_entry {
                            let raw = fEntry.as_str();
                            let mut parts = raw.split(':');
                            let fnm = parts.next().unwrap_or("").trim().trim_matches('"').to_string();
                            let ftp = parts.next().unwrap_or("").trim().trim_matches('"').to_string();
                            fields.push(TableFieldDto { name: fnm, fieldType: ftp });
                        }
                    }
                }
                ThreadRule::indexes_block => {
                    for idx in child.into_inner() {
                        if idx.as_rule() == ThreadRule::index_decl {
                            indexes.push(idx.as_str().trim().to_string());
                        }
                    }
                }
                ThreadRule::relations_block => {
                    for rel in child.into_inner() {
                        if rel.as_rule() == ThreadRule::db_relation {
                            let mut colRefs = Vec::new();
                            let mut relType = String::from("1:N");
                            for rChild in rel.into_inner() {
                                if rChild.as_rule() == ThreadRule::column_ref {
                                    colRefs.push(rChild.as_str().to_string());
                                } else if rChild.as_rule() == ThreadRule::cardinality {
                                    relType = rChild.as_str().to_string();
                                }
                            }
                            if colRefs.len() == 2 {
                                let lParts: Vec<&str> = colRefs[0].split('.').collect();
                                let rParts: Vec<&str> = colRefs[1].split('.').collect();
                                relations.push(RelationDto {
                                    leftTable: lParts.first().copied().unwrap_or("").to_string(),
                                    leftColumn: lParts.get(1).copied().unwrap_or("").to_string(),
                                    relationType: relType,
                                    rightTable: rParts.first().copied().unwrap_or("").to_string(),
                                    rightColumn: rParts.get(1).copied().unwrap_or("").to_string(),
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    TableDto { name: tName, decorators, fields, indexes, relations }
}

fn extractProtocolEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ProtocolDto
{
    let mut pName = String::new();
    let mut notes = Vec::new();
    let mut diagrams = Vec::new();
    let mut channels = Vec::new();

    for child in pair.clone().into_inner() {
        match child.as_rule() {
            ThreadRule::ident => {
                if pName.is_empty() {
                    pName = child.as_str().to_string();
                }
            }
            ThreadRule::note_block => {
                notes.push(extractNote(&child));
            }
            ThreadRule::diagram_block => {
                diagrams.push(extractDiagram(&child));
            }
            ThreadRule::channel_block => {
                channels.push(extractChannelEntity(&child));
            }
            _ => {}
        }
    }

    ProtocolDto { name: pName, notes, diagrams, channels }
}

fn extractChannelEntity(pair: &pest::iterators::Pair<'_, ThreadRule>) -> ChannelDto
{
    let mut chName = String::new();
    let mut decorators = Vec::new();
    let mut pattern = String::new();
    let mut transport = String::new();
    let mut sender = String::new();
    let mut receiver = String::new();
    let mut payload = String::new();
    let mut errors = Vec::new();

    let mut chItems = Vec::new();
    for child in pair.clone().into_inner() {
        if child.as_rule() == ThreadRule::channel_body_item {
            chItems.extend(child.into_inner());
        } else {
            chItems.push(child);
        }
    }

    for child in chItems {
        if isDecoratorRule(child.as_rule()) {
            decorators.push(extractDecorator(&child));
        } else {
            match child.as_rule() {
                ThreadRule::ident => {
                    if chName.is_empty() {
                        chName = child.as_str().to_string();
                    }
                }
                ThreadRule::pattern_stmt => {
                    pattern = child.as_str().replace("Pattern", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::transport_stmt => {
                    transport = child.as_str().replace("Transport", "").trim().trim_matches('"').to_string();
                }
                ThreadRule::sender_stmt => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            sender = stmt.as_str().trim_matches('"').to_string();
                        }
                    }
                }
                ThreadRule::receiver_stmt => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            receiver = stmt.as_str().trim_matches('"').to_string();
                        }
                    }
                }
                ThreadRule::payload_stmt => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            payload = stmt.as_str().trim_matches('"').to_string();
                        }
                    }
                }
                ThreadRule::errors_block => {
                    for stmt in child.into_inner() {
                        if stmt.as_rule() == ThreadRule::string_lit {
                            errors.push(stmt.as_str().trim_matches('"').to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ChannelDto {
        name: chName,
        decorators,
        pattern,
        transport,
        sender,
        receiver,
        payload,
        errors,
    }
}

fn extractFabricGroup(pair: &pest::iterators::Pair<'_, FabricRule>) -> FabricGroupDto
{
    let mut gName = String::new();
    let mut members = Vec::new();

    for child in pair.clone().into_inner() {
        if child.as_rule() == FabricRule::string_lit {
            gName = child.as_str().trim_matches('"').to_string();
        } else if child.as_rule() == FabricRule::entity_ref {
            members.push(extractFabricEntityRef(&child));
        }
    }

    FabricGroupDto { name: gName, members }
}

fn extractFabricConnection(pair: &pest::iterators::Pair<'_, FabricRule>) -> FabricConnectionDto
{
    let mut refs = Vec::new();
    let mut lbl = String::new();

    for child in pair.clone().into_inner() {
        if child.as_rule() == FabricRule::entity_ref {
            refs.push(extractFabricEntityRef(&child));
        } else if child.as_rule() == FabricRule::string_lit {
            lbl = child.as_str().trim_matches('"').to_string();
        }
    }

    let defaultRef = FabricEntityRefDto { kind: String::new(), path: String::new() };
    let src = refs.first().cloned().unwrap_or(defaultRef.clone());
    let tgt = refs.get(1).cloned().unwrap_or(defaultRef);

    FabricConnectionDto { source: src, target: tgt, label: lbl }
}

fn extractFabricEntityRef(pair: &pest::iterators::Pair<'_, FabricRule>) -> FabricEntityRefDto
{
    let mut k = String::new();
    let mut p = String::new();

    for child in pair.clone().into_inner() {
        if child.as_rule() == FabricRule::entity_kind {
            k = child.as_str().to_string();
        } else if child.as_rule() == FabricRule::scoped_path {
            p = child.as_str().to_string();
        }
    }

    FabricEntityRefDto { kind: k, path: p }
}
