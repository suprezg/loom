/*
File Name: wiki.ts
Purpose: TypeScript interfaces matching Loom's JSON AST output for Thread and Fabric documents.
*/

export interface StepDto {
  keyword: string;
  text: string;
}

export interface DecoratorDto {
  decoratorType: string;
  target: string;
}

export interface DiagramDto {
  name: string;
  content: string;
}

export interface ScenarioDto {
  name: string;
  title?: string;
  isOutline: boolean;
  decorators: DecoratorDto[];
  steps: StepDto[];
  examples: string[];
}

export interface RuleDto {
  name: string;
  scenarios: ScenarioDto[];
}

export interface FeatureDto {
  name: string;
  notes: string[];
  diagrams: DiagramDto[];
  backgroundSteps: StepDto[];
  rules: RuleDto[];
  scenarios: ScenarioDto[];
}

export interface ContractDto {
  name: string;
  decorators: DecoratorDto[];
  signature: string;
  requires: string;
  ensures: string;
  preconditions: string[];
  postconditions: string[];
  process: string[];
  errors: string[];
}

export interface ModelDto {
  name: string;
  decorators: DecoratorDto[];
  modelType: string;
  members: string[][];
}

export interface ComponentDto {
  name: string;
  notes: string[];
  diagrams: DiagramDto[];
  invariants: string[];
  contracts: ContractDto[];
  models: ModelDto[];
}

export interface TableFieldDto {
  name: string;
  fieldType: string;
}

export interface RelationDto {
  leftTable: string;
  leftColumn: string;
  relationType: string;
  rightTable: string;
  rightColumn: string;
}

export interface TableDto {
  name: string;
  decorators: DecoratorDto[];
  fields: TableFieldDto[];
  indexes: string[];
  relations: RelationDto[];
}

export interface StorageDto {
  name: string;
  engine: string;
  notes: string[];
  diagrams: DiagramDto[];
  tables: TableDto[];
}

export interface ChannelDto {
  name: string;
  decorators: DecoratorDto[];
  pattern: string;
  transport: string;
  sender: string;
  receiver: string;
  payload: string;
  errors: string[];
}

export interface ProtocolDto {
  name: string;
  notes: string[];
  diagrams: DiagramDto[];
  channels: ChannelDto[];
}

export interface ThreadDocumentDto {
  features: FeatureDto[];
  components: ComponentDto[];
  storages: StorageDto[];
  protocols: ProtocolDto[];
}

export interface FabricEntityRefDto {
  kind: string;
  path: string;
}

export interface FabricGroupDto {
  name: String;
  members: FabricEntityRefDto[];
}

export interface FabricConnectionDto {
  source: FabricEntityRefDto;
  target: FabricEntityRefDto;
  label: string;
}

export interface FabricDocumentDto {
  system: string;
  groups: FabricGroupDto[];
  connections: FabricConnectionDto[];
}

export interface WikiData {
  thread: ThreadDocumentDto;
  fabric?: FabricDocumentDto;
}

export type EntityKind = 'Feature' | 'Component' | 'Storage' | 'Protocol';
export type MemberKind = 'Scenario' | 'Scenario Outline' | 'Model' | 'Contract' | 'Table' | 'Channel';

export interface SearchResultItem {
  id: string;
  type: 'entity' | 'member';
  kind: EntityKind | MemberKind;
  name: string;
  parentEntityName: string;
  title?: string;
  description?: string;
}
