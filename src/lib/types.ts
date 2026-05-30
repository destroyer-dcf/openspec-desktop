export type Artifact = {
  name: string;
  present: boolean;
  path: string | null;
};

export type TaskProgress = {
  complete: number;
  total: number;
};

export type Change = {
  name: string;
  archived: boolean;
  status: "Pending" | "Ready" | "Blocked";
  artifacts: Artifact[];
  tasks: TaskProgress | null;
  why_summary: string;
  archived_at: string | null;
  archived_documents: { name: string; path: string }[];
};

export type ProjectState = {
  root_path: string;
  openspec_path: string;
  config: { schema?: string; contexto?: string };
  active_changes: Change[];
  archived_changes: Change[];
  specs: string[];
};

export type ProjectHandle = {
  path: string;
  name: string;
  state: ProjectState;
};

export type OpenProjectResponse =
  | { status: "loaded"; project: ProjectState }
  | { status: "needs_init"; path: string };

export type InitProjectInput = {
  path: string;
  name: string;
  schema?: string;
  language?: string;
  audience?: string;
  domain?: string;
  description?: string;
  stack: string;
  architecture?: string;
  deployment_flow?: string;
  ai_provider: string[];
  proposal_rules?: string[];
  specs_rules?: string[];
  design_rules?: string[];
  tasks_rules?: string[];
};

export type ProposalType = "feature" | "bug";

export type Proposal = {
  name: string;
  proposal_type: ProposalType;
  created_at: string;
  status: "active" | "archived";
  path: string;
  summary?: string;
};

export type ProposalList = {
  active: Proposal[];
  archived: Proposal[];
};

export type ProposalDetail = {
  proposal: Proposal;
  content: string;
};
