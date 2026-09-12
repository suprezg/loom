/*
File Name: SigmaGraphEngine.tsx
Purpose: Graphology builder utility and Sigma.js graph layout construction helper.
*/

import Graphology from 'graphology';
import type { ThreadDocumentDto } from '../../types/wiki';

export const buildGraphologyInstance = (
  thread: ThreadDocumentDto,
  fgColor: string
): Graphology => {
  const graph = new Graphology({ type: 'directed' });

  let angle = 0;
  const entityRadius = 380;
  const totalEntities =
    thread.features.length +
    thread.components.length +
    thread.storages.length +
    thread.protocols.length;

  const angleStep = (2 * Math.PI) / (totalEntities || 1);

  const placeEntityHub = (
    entityName: string,
    entityKind: string,
    color: string,
    members: { name: string; kind: string }[]
  ) => {
    const entityX = Math.cos(angle) * entityRadius;
    const entityY = Math.sin(angle) * entityRadius;
    angle += angleStep;

    graph.addNode(entityName, {
      label: `${entityName} [${entityKind}]`,
      size: 22,
      color: color,
      x: entityX,
      y: entityY,
      nodeType: 'entity',
      entityName
    });

    const memberCount = members.length;
    members.forEach((m, idx) => {
      const mAngle = (2 * Math.PI * idx) / (memberCount || 1);
      const memberRadius = 90;
      const mX = entityX + Math.cos(mAngle) * memberRadius;
      const mY = entityY + Math.sin(mAngle) * memberRadius;
      const memberNodeId = `${entityName}::${m.name}`;

      if (!graph.hasNode(memberNodeId)) {
        graph.addNode(memberNodeId, {
          label: `${m.name}`,
          size: 11,
          color: color,
          x: mX,
          y: mY,
          nodeType: 'member',
          entityName,
          memberName: m.name
        });
      }

      if (!graph.hasEdge(entityName, memberNodeId)) {
        graph.addEdge(entityName, memberNodeId, {
          size: 2,
          color: fgColor,
          type: 'line'
        });
      }
    });
  };

  thread.features.forEach((f) => {
    const scenarios = f.rules.flatMap((r) => r.scenarios).concat(f.scenarios);
    placeEntityHub(f.name, 'Feature', '#EC5B38', scenarios.map((s) => ({ name: s.name, kind: s.isOutline ? 'Scenario Outline' : 'Scenario' })));
  });

  thread.components.forEach((c) => {
    const members = [
      ...c.models.map((m) => ({ name: m.name, kind: 'Model' })),
      ...c.contracts.map((ct) => ({ name: ct.name, kind: 'Contract' }))
    ];
    placeEntityHub(c.name, 'Component', '#3B82F6', members);
  });

  thread.storages.forEach((s) => {
    placeEntityHub(s.name, 'Storage', '#10B981', s.tables.map((t) => ({ name: t.name, kind: 'Table' })));
  });

  thread.protocols.forEach((p) => {
    placeEntityHub(p.name, 'Protocol', '#F59E0B', p.channels.map((ch) => ({ name: ch.name, kind: 'Channel' })));
  });

  const addDecoratorEdges = (sourceNodeId: string, decorators: { decoratorType: string; target: string }[]) => {
    decorators.forEach((dec) => {
      if (dec.decoratorType === 'diagram') return;

      let targetNodeId = dec.target;
      if (!graph.hasNode(targetNodeId)) {
        const entityPart = dec.target.split('::')[0];
        if (graph.hasNode(entityPart)) {
          targetNodeId = entityPart;
        }
      }

      if (graph.hasNode(targetNodeId) && sourceNodeId !== targetNodeId) {
        const edgeId = `dec-${sourceNodeId}-${targetNodeId}`;
        if (!graph.hasEdge(sourceNodeId, targetNodeId) && !graph.hasEdge(edgeId)) {
          try {
            graph.addEdgeWithKey(edgeId, sourceNodeId, targetNodeId, {
              size: 2,
              color: fgColor,
              type: 'arrow'
            });
          } catch {
            // Ignore duplicate key
          }
        }
      }
    });
  };

  thread.features.forEach((f) => {
    f.rules.flatMap((r) => r.scenarios).concat(f.scenarios).forEach((sc) => {
      addDecoratorEdges(`${f.name}::${sc.name}`, sc.decorators);
    });
  });

  thread.components.forEach((c) => {
    c.contracts.forEach((ct) => addDecoratorEdges(`${c.name}::${ct.name}`, ct.decorators));
    c.models.forEach((m) => addDecoratorEdges(`${c.name}::${m.name}`, m.decorators));
  });

  thread.storages.forEach((s) => {
    s.tables.forEach((t) => addDecoratorEdges(`${s.name}::${t.name}`, t.decorators));
  });

  thread.protocols.forEach((p) => {
    p.channels.forEach((ch) => addDecoratorEdges(`${p.name}::${ch.name}`, ch.decorators));
  });

  return graph;
};
