export interface NodeResult {
    id: string;
    name: string;
    kind: string;
    description: string | null;
    version: string | null;
}
export interface SolveResult {
    query: string;
    bundle: string;
    confidence: number;
    total_matches: number;
    nodes: NodeResult[];
}
export interface DecideResult {
    tree_id: string;
    path: string[];
    rationale: string;
    recommendations: Array<{
        node_id: string;
        confidence: number;
    }>;
    agent_instructions: string | null;
}
export declare function solve(bundlePath: string, query: string): SolveResult;
export declare function decide(bundlePath: string, query: string, context?: Record<string, string>): DecideResult | null;
export declare function verify(bundlePath: string, artifact?: string): any;
export declare function dump(bundlePath: string): any;
export declare function reason(bundlePath: string, query: string): string;
