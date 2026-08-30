import * as vscode from 'vscode';

interface BlockContext {
  entityType?: 'Feature' | 'Component' | 'Storage' | 'Protocol';
  entityName?: string;
  subBlock?: string;
  inBraces: boolean;
}

/**
 * Analyzes document structure up to position to determine active entity context and sub-block level.
 */
function getThreadContext(document: vscode.TextDocument, position: vscode.Position): BlockContext {
  let entityType: 'Feature' | 'Component' | 'Storage' | 'Protocol' | undefined;
  let entityName: string | undefined;
  let subBlock: string | undefined;

  let braceCount = 0;
  const textUpToPosition = document.getText(new vscode.Range(new vscode.Position(0, 0), position));
  const lines = textUpToPosition.split('\n');

  for (let i = 0; i < lines.length; i++) {
    const rawLine = lines[i];
    // Strip line comments and double-quoted strings for brace balance calculation
    const line = rawLine.replace(/#.*$/, '').replace(/"[^"]*"/g, '""');

    // Detect top-level entity declarations when braceCount is 0
    const entityMatch = line.match(/\b(Feature|Component|Storage|Protocol)\s+([A-Za-z_0-9]+)/i);
    if (entityMatch && braceCount === 0) {
      const typeStr = entityMatch[1].toLowerCase();
      if (typeStr === 'feature') entityType = 'Feature';
      else if (typeStr === 'component') entityType = 'Component';
      else if (typeStr === 'storage') entityType = 'Storage';
      else if (typeStr === 'protocol') entityType = 'Protocol';
      entityName = entityMatch[2];
      subBlock = undefined;
    }

    // Identify current active sub-block inside active entity
    if (entityType) {
      if (/\bBackground\b/i.test(line)) subBlock = 'Background';
      else if (/\bRule\b/i.test(line)) subBlock = 'Rule';
      else if (/\bScenario\s+Outline\b/i.test(line)) subBlock = 'Scenario Outline';
      else if (/\bScenario\b/i.test(line)) subBlock = 'Scenario';
      else if (/\bExamples\b/i.test(line)) subBlock = 'Examples';
      else if (/\bInvariants\b/i.test(line)) subBlock = 'Invariants';
      else if (/\bModel\b/i.test(line)) subBlock = 'Model';
      else if (/\bMembers\b/i.test(line)) subBlock = 'Members';
      else if (/\bContract\b/i.test(line)) subBlock = 'Contract';
      else if (/\bPrecondition\b/i.test(line)) subBlock = 'Precondition';
      else if (/\bPostcondition\b/i.test(line)) subBlock = 'Postcondition';
      else if (/\bProcess\b/i.test(line)) subBlock = 'Process';
      else if (/\bTable\b/i.test(line)) subBlock = 'Table';
      else if (/\bFields\b/i.test(line)) subBlock = 'Fields';
      else if (/\bIndexes\b/i.test(line)) subBlock = 'Indexes';
      else if (/\bRelations\b/i.test(line)) subBlock = 'Relations';
      else if (/\bChannel\b/i.test(line)) subBlock = 'Channel';
      else if (/\bErrors\b/i.test(line)) subBlock = 'Errors';
    }

    // Update brace depth balance
    for (const char of line) {
      if (char === '{') {
        braceCount++;
      } else if (char === '}') {
        braceCount = Math.max(0, braceCount - 1);
        if (braceCount === 0) {
          entityType = undefined;
          entityName = undefined;
          subBlock = undefined;
        } else if (braceCount === 1) {
          subBlock = undefined;
        }
      }
    }
  }

  return {
    entityType,
    entityName,
    subBlock,
    inBraces: braceCount > 0
  };
}

export function activate(context: vscode.ExtensionContext) {
  // Autocomplete provider for .fabric files
  const fabricProvider = vscode.languages.registerCompletionItemProvider(
    'loom-fabric',
    {
      provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
        const items: vscode.CompletionItem[] = [];

        // Regular Autocompletion (Keywords & Domain entities)
        const sysKeyword = new vscode.CompletionItem('system', vscode.CompletionItemKind.Keyword);
        sysKeyword.insertText = new vscode.SnippetString('system "${1:SystemName}"');
        items.push(sysKeyword);

        const groupKeyword = new vscode.CompletionItem('group', vscode.CompletionItemKind.Keyword);
        groupKeyword.insertText = new vscode.SnippetString('group "${1:GroupName}"\n{\n    $0\n}');
        items.push(groupKeyword);

        ['Feature', 'Component', 'Storage', 'Protocol'].forEach(kind => {
          const kindItem = new vscode.CompletionItem(kind + '.', vscode.CompletionItemKind.Class);
          kindItem.insertText = kind + '.';
          kindItem.detail = `Reference ${kind} entity in Fabric blueprint`;
          items.push(kindItem);
        });

        return items;
      }
    },
    '.', '$'
  );

  // Context-aware Autocomplete provider for .thread files
  const threadProvider = vscode.languages.registerCompletionItemProvider(
    'loom-thread',
    {
      provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
        const ctx = getThreadContext(document, position);
        const items: vscode.CompletionItem[] = [];

        const addKeyword = (label: string, insertText?: string | vscode.SnippetString, detail?: string) => {
          const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Keyword);
          if (insertText) {
            item.insertText = insertText;
          }
          if (detail) {
            item.detail = detail;
          }
          items.push(item);
        };

        const addSnippet = (label: string, snippetStr: string, detail?: string) => {
          const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Snippet);
          item.insertText = new vscode.SnippetString(snippetStr);
          if (detail) {
            item.detail = detail;
          }
          items.push(item);
        };

        const addBlocks = () => {
          addSnippet('!Note', '!Note [[ ${1:Note description} ]]', 'Documentation Note Block');
          addSnippet('!Diagram', '!Diagram ${1:DiagramName}\n[[\n```mermaid\n${2:sequenceDiagram}\n${3:Alice ->> Bob: Request}\n```\n]]', 'Mermaid Diagram Block');
        };

        // REGULAR CONTEXT-AWARE KEYWORD AUTOCOMPLETION
        if (!ctx.inBraces || !ctx.entityType) {
          // Top-Level Context
          addSnippet('Feature', 'Feature ${1:FeatureName}\n{\n    $0\n}', 'Feature Entity Header');
          addSnippet('Component', 'Component ${1:ComponentName}\n{\n    $0\n}', 'Component Entity Header');
          addSnippet('Storage', 'Storage ${1:StorageName}\n{\n    $0\n}', 'Storage Entity Header');
          addSnippet('Protocol', 'Protocol ${1:ProtocolName}\n{\n    $0\n}', 'Protocol Entity Header');
          return items;
        }

        // Inside Feature Entity Context
        if (ctx.entityType === 'Feature') {
          if (ctx.subBlock === 'Scenario' || ctx.subBlock === 'Scenario Outline' || ctx.subBlock === 'Background') {
            addKeyword('Given', new vscode.SnippetString('Given "${1:initial state}"'), 'Behavioral Given Step');
            addKeyword('When', new vscode.SnippetString('When "${1:action performed}"'), 'Behavioral When Step');
            addKeyword('Then', new vscode.SnippetString('Then "${1:expected outcome}"'), 'Behavioral Then Step');
            addKeyword('And', new vscode.SnippetString('And "${1:step}"'), 'Behavioral And Step');
            addKeyword('But', new vscode.SnippetString('But "${1:step}"'), 'Behavioral But Step');
            if (ctx.subBlock === 'Scenario Outline') {
              addSnippet('Examples', 'Examples\n{\n    | "${1:param1}" | "${2:param2}" |\n    | "${3:val1}"   | "${4:val2}"   |\n}', 'Examples Matrix');
            }
            addBlocks();
          } else if (ctx.subBlock === 'Rule') {
            addSnippet('Scenario', 'Scenario "${1:Scenario Name}"\n{\n    Given "${2:initial state}"\n    When "${3:action}"\n    Then "${4:expected result}"\n}', 'Scenario Block');
            addSnippet('Scenario Outline', 'Scenario Outline "${1:Outline Name}"\n{\n    Given "${2:step with <param>}"\n    When "${3:action}"\n    Then "${4:result}"\n\n    Examples\n    {\n        | "${5:param}" |\n        | "${6:val}"   |\n    }\n}', 'Scenario Outline Block');
            addBlocks();
          } else {
            // Feature Root Level
            addSnippet('Background', 'Background\n{\n    Given "${1:initial state}"\n}', 'Background Block');
            addSnippet('Rule', 'Rule "${1:Rule Description}"\n{\n    Scenario "${2:Scenario Name}"\n    {\n        Given "${3:condition}"\n        When "${4:action}"\n        Then "${5:result}"\n    }\n}', 'Rule Block');
            addSnippet('Scenario', 'Scenario "${1:Scenario Name}"\n{\n    Given "${2:initial state}"\n    When "${3:action}"\n    Then "${4:expected result}"\n}', 'Scenario Block');
            addSnippet('Scenario Outline', 'Scenario Outline "${1:Outline Name}"\n{\n    Given "${2:step with <param>}"\n    When "${3:action}"\n    Then "${4:result}"\n\n    Examples\n    {\n        | "${5:param}" |\n        | "${6:val}"   |\n    }\n}', 'Scenario Outline Block');
            addBlocks();
          }
          return items;
        }

        // Inside Component Entity Context
        if (ctx.entityType === 'Component') {
          if (ctx.subBlock === 'Model') {
            addKeyword('Type', new vscode.SnippetString('Type "${1:Struct}"'), 'Model Type Statement');
            addSnippet('Members', 'Members\n{\n    "${1:fieldName}": "${2:DataType}"\n}', 'Model Members Block');
          } else if (ctx.subBlock === 'Members') {
            addSnippet('MemberField', '"${1:fieldName}": "${2:DataType}"', 'Member Field Definition');
          } else if (ctx.subBlock === 'Contract') {
            addKeyword('Signature', new vscode.SnippetString('Signature "${1:signature}" -> "${2:ReturnType}"'), 'Contract Signature');
            addKeyword('Requires', new vscode.SnippetString('Requires "${1:Precondition statement}"'), 'Contract Requires');
            addKeyword('Ensures', new vscode.SnippetString('Ensures "${1:Postcondition statement}"'), 'Contract Ensures');
            addSnippet('Precondition', 'Precondition\n{\n    "${1:Condition}"\n}', 'Contract Precondition Block');
            addSnippet('Postcondition', 'Postcondition\n{\n    "${1:Condition}"\n}', 'Contract Postcondition Block');
            addSnippet('Process', 'Process\n{\n    1. "${1:Step 1}"\n}', 'Contract Process Block');
            addSnippet('Errors', 'Errors\n{\n    "${1:Error description}"\n}', 'Contract Errors Block');
            addBlocks();
          } else if (ctx.subBlock === 'Process') {
            addSnippet('ProcessStep', '1. "${1:Process step description}"', 'Indexed Process Step');
          } else {
            // Component Root Level
            addSnippet('Invariants', 'Invariants\n{\n    "${1:Security or state invariant}"\n}', 'Component Invariants Block');
            addSnippet('Model', 'Model ${1:ModelName}\n{\n    Type "${2:Struct}"\n    Members\n    {\n        "${3:fieldName}": "${4:DataType}"\n    }\n}', 'Component Model Block');
            addSnippet('Contract', 'Contract ${1:contractName}\n{\n    Signature "${2:signature}" -> "${3:ReturnType}"\n    Requires "${4:Precondition}"\n    Ensures "${5:Postcondition}"\n    Process\n    {\n        1. "${6:Step 1}"\n    }\n}', 'Component Contract Block');
            addBlocks();
          }
          return items;
        }

        // Inside Storage Entity Context
        if (ctx.entityType === 'Storage') {
          if (ctx.subBlock === 'Table') {
            addSnippet('Fields', 'Fields\n{\n    "${1:id}": "${2:UUID}"\n}', 'Table Fields Block');
            addSnippet('Indexes', 'Indexes\n{\n    ${1:IdxName} (${2:columnName})\n}', 'Table Indexes Block');
            addSnippet('Relations', 'Relations\n{\n    ${1:table1.col} 1:N ${2:table2.col}\n}', 'Table Relations Block');
            addBlocks();
          } else if (ctx.subBlock === 'Fields') {
            addSnippet('FieldsEntry', '"${1:colName}": "${2:DataType}"', 'Table Field Definition');
          } else if (ctx.subBlock === 'Indexes') {
            addSnippet('IndexDeclaration', '${1:IdxName} (${2:columnName})', 'Table Index Declaration');
          } else if (ctx.subBlock === 'Relations') {
            addSnippet('RelationsDeclaration', '${1:table1.col} 1:N ${2:table2.col}', 'Table Relation Statement');
          } else {
            // Storage Root Level
            addKeyword('Engine', new vscode.SnippetString('Engine "${1:PostgreSQL}"'), 'Storage Database Engine');
            addSnippet('Table', 'Table ${1:tableName}\n{\n    Fields\n    {\n        "${2:id}": "${3:UUID}"\n    }\n\n    Indexes\n    {\n        ${4:IdxName} (${5:id})\n    }\n}', 'Storage Table Block');
            addBlocks();
          }
          return items;
        }

        // Inside Protocol Entity Context
        if (ctx.entityType === 'Protocol') {
          if (ctx.subBlock === 'Channel') {
            addKeyword('Pattern', new vscode.SnippetString('Pattern "${1:Publish-Subscribe}"'), 'Channel Pattern Statement');
            addKeyword('Transport', new vscode.SnippetString('Transport "${1:gRPC}"'), 'Channel Transport Statement');
            addKeyword('Sender', new vscode.SnippetString('Sender "${1:SenderComponent}"'), 'Channel Sender Statement');
            addKeyword('Receiver', new vscode.SnippetString('Receiver "${1:ReceiverComponent}"'), 'Channel Receiver Statement');
            addKeyword('Payload', new vscode.SnippetString('Payload "${1:PayloadModel}"'), 'Channel Payload Statement');
            addSnippet('Errors', 'Errors\n{\n    "${1:Error description}"\n}', 'Channel Errors Block');
            addBlocks();
          } else {
            // Protocol Root Level
            addSnippet('Channel', 'Channel ${1:ChannelName}\n{\n    Pattern "${2:Request-Response}"\n    Transport "${3:gRPC}"\n    Sender "${4:SenderComponent}"\n    Receiver "${5:ReceiverComponent}"\n    Payload "${6:PayloadModel}"\n}', 'Protocol Channel Block');
            addBlocks();
          }
          return items;
        }

        return items;
      }
    },
    ' ', ':', '$', '@', '!'
  );

  context.subscriptions.push(fabricProvider, threadProvider);
}

export function deactivate() {}