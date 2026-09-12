/*
File Name: SearchModalIndex.ts
Purpose: Helper utility for constructing search index for entities and members.
*/

import type { WikiData, SearchResultItem } from '../../types/wiki';

export const buildSearchIndex = (wikiData: WikiData): SearchResultItem[] => {
  const items: SearchResultItem[] = [];

  if (!wikiData.thread) return items;

  // 1. Features & Scenarios
  wikiData.thread.features.forEach((feature) => {
    items.push({
      id: `entity-feature-${feature.name}`,
      type: 'entity',
      kind: 'Feature',
      name: feature.name,
      parentEntityName: feature.name,
      description: feature.notes[0] || 'Feature Entity Specification'
    });

    feature.rules.forEach((rule) => {
      rule.scenarios.forEach((sc) => {
        items.push({
          id: `member-scenario-${sc.name}`,
          type: 'member',
          kind: sc.isOutline ? 'Scenario Outline' : 'Scenario',
          name: sc.name,
          parentEntityName: feature.name,
          title: sc.title,
          description: sc.title || `Scenario under ${feature.name}`
        });
      });
    });

    feature.scenarios.forEach((sc) => {
      items.push({
        id: `member-scenario-${sc.name}`,
        type: 'member',
        kind: sc.isOutline ? 'Scenario Outline' : 'Scenario',
        name: sc.name,
        parentEntityName: feature.name,
        title: sc.title,
        description: sc.title || `Scenario under ${feature.name}`
      });
    });
  });

  // 2. Components, Contracts & Models
  wikiData.thread.components.forEach((comp) => {
    items.push({
      id: `entity-component-${comp.name}`,
      type: 'entity',
      kind: 'Component',
      name: comp.name,
      parentEntityName: comp.name,
      description: comp.notes[0] || 'Component Entity Engine'
    });

    comp.contracts.forEach((c) => {
      items.push({
        id: `member-contract-${c.name}`,
        type: 'member',
        kind: 'Contract',
        name: c.name,
        parentEntityName: comp.name,
        title: c.signature,
        description: c.signature || `Contract under ${comp.name}`
      });
    });

    comp.models.forEach((m) => {
      items.push({
        id: `member-model-${m.name}`,
        type: 'member',
        kind: 'Model',
        name: m.name,
        parentEntityName: comp.name,
        description: `${m.modelType} Model under ${comp.name}`
      });
    });
  });

  // 3. Storages & Tables
  wikiData.thread.storages.forEach((st) => {
    items.push({
      id: `entity-storage-${st.name}`,
      type: 'entity',
      kind: 'Storage',
      name: st.name,
      parentEntityName: st.name,
      description: `${st.engine} Engine under ${st.name}`
    });

    st.tables.forEach((t) => {
      items.push({
        id: `member-table-${t.name}`,
        type: 'member',
        kind: 'Table',
        name: t.name,
        parentEntityName: st.name,
        description: `Database Table under ${st.name}`
      });
    });
  });

  // 4. Protocols & Channels
  wikiData.thread.protocols.forEach((proto) => {
    items.push({
      id: `entity-protocol-${proto.name}`,
      type: 'entity',
      kind: 'Protocol',
      name: proto.name,
      parentEntityName: proto.name,
      description: proto.notes[0] || 'Protocol Entity Stream'
    });

    proto.channels.forEach((ch) => {
      items.push({
        id: `member-channel-${ch.name}`,
        type: 'member',
        kind: 'Channel',
        name: ch.name,
        parentEntityName: proto.name,
        description: `${ch.pattern} channel under ${proto.name}`
      });
    });
  });

  return items;
};
