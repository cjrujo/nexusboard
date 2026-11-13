import assert from 'node:assert/strict';
import { NexusYamlEngine } from './engine.mjs';

const engine = new NexusYamlEngine();

// Test 1: Disjoint task concurrent edits (Human edits CORE-101, Agent edits CORE-102)
const base = [
  { id: 'CORE-101', title: 'Task 1', status: 'ready', priority: 'P1', progress: 20 },
  { id: 'CORE-102', title: 'Task 2', status: 'ready', priority: 'P1', progress: 10 }
];
const humanState = [
  { id: 'CORE-101', title: 'Task 1', status: 'in_progress', priority: 'P0', progress: 50 },
  { id: 'CORE-102', title: 'Task 2', status: 'ready', priority: 'P1', progress: 10 }
];
const agentState = [
  { id: 'CORE-101', title: 'Task 1', status: 'ready', priority: 'P1', progress: 20 },
  { id: 'CORE-102', title: 'Task 2', status: 'done', priority: 'P1', progress: 100 }
];

const res1 = engine.mergeThreeWayProject(base, humanState, agentState, 'pi-agent');
assert.equal(res1.conflicts.length, 0, 'Disjoint task updates should merge with 0 conflicts');
assert.equal(res1.mergedTasks.find(t => t.id === 'CORE-101').status, 'in_progress');
assert.equal(res1.mergedTasks.find(t => t.id === 'CORE-102').status, 'done');

// Test 2: Same-task disjoint field edits (Human changes priority, Agent updates progress)
const humanSameTask = [{ id: 'CORE-101', title: 'Task 1', status: 'ready', priority: 'P0', progress: 20 }];
const agentSameTask = [{ id: 'CORE-101', title: 'Task 1', status: 'ready', priority: 'P1', progress: 85 }];
const res2 = engine.mergeThreeWayProject([base[0]], humanSameTask, agentSameTask, 'claude-code');
assert.equal(res2.conflicts.length, 0, 'Disjoint field updates on same task should auto-merge');
assert.equal(res2.mergedTasks[0].priority, 'P0');
assert.equal(res2.mergedTasks[0].progress, 85);

// Test 3: Malformed YAML Quarantine Guard
const badYaml = "schema: nexusboard-project/v1.0\n\ttasks: []";
const check = engine.validateYamlStructure(badYaml, 'core-platform');
assert.equal(check.valid, false);
assert.equal(check.line, 2);

console.log('PASS: All 3-Way Task-Level CAS Merger, Disjoint Field Merge, and Malformed YAML Quarantine tests succeeded.');
