export type AppTab = "overview" | "resume" | "messages" | "export" | "tracker";

export interface GenerateRequest {
  company: string;
  role: string;
  source: string;
  baseline: "1pg" | "2pg";
  jdText: string;
}

export interface FitBreakdown {
  roleMatch: number;
  stackMatch: number;
  scaleMatch: number;
  rigorMatch: number;
  signalBoost: number;
  total: number;
  whyMatch: string[];
  gaps: string[];
}

export interface TailorEdit {
  kind: string;
  targetSection: string;
  reason: string;
  provenanceIds: string[];
}

export interface TailorPlan {
  edits: TailorEdit[];
  maxResumeEdits: number;
  maxBulletSwaps: number;
}

export interface BulletCandidate {
  id: string;
  text: string;
  tags: string[];
  trackHint: string;
  reason: string;
  approved: boolean;
  score: number;
}

export interface Messages {
  recruiter: string;
  hiringManager: string;
  coverShort: string;
}

export interface TruthReport {
  passed: boolean;
  violations: string[];
  unknownTools: string[];
  claimIssues: string[];
  provenanceComplete: boolean;
}

export interface TrackerRow {
  date: string;
  company: string;
  role: string;
  source: string;
  track: string;
  fitTotal: number;
  status: string;
  nextAction: string;
  packetDir: string;
}

export interface PacketDetail {
  packetDir: string;
  extractionSource?: "deterministic" | "llm_merged";
  extractionDiagnostics?: {
    summarizeAttempted: boolean;
    summarizeMerged: boolean;
    summarizeFallbackReasons: string[];
  };
  extractedKeywords: string[];
  extractedTools: string[];
  extractedRequirements: string[];
  fitBreakdown: FitBreakdown;
  track: string;
  trackScores: [string, number, string[]][];
  tailorPlan: TailorPlan;
  bulletCandidates: BulletCandidate[];
  messages: Messages;
  resume1pg: string;
  resume2pg?: string;
  diff: string;
  trackerRow: TrackerRow;
  truthReport: TruthReport;
}

export interface GenerateResponse {
  packetDir: string;
  fitTotal: number;
  track: string;
  filesWritten: string[];
  truthPassed: boolean;
  packetDetail: PacketDetail;
}

export interface JobSummary {
  id: string;
  company: string;
  role: string;
  source: string;
  baseline: string;
  track?: string;
  fitTotal?: number;
  status: string;
  nextAction?: string;
  notes?: string;
  outputDir?: string;
  updatedAt: string;
}

export interface SettingsModel {
  allowUnapproved: boolean;
  llmEnabled: boolean;
  llmProvider: string;
  llmBaseUrl: string;
  llmModel: string;
  llmAllowedTasks: string[];
}

export interface ExportResponse {
  ok: boolean;
  outputPath?: string;
  message: string;
}

export interface UpdateJobStatusResponse {
  ok: boolean;
  id: string;
  status: string;
  nextAction?: string;
  notes?: string;
}

export interface MutationResponse {
  ok: boolean;
  message: string;
  updatedAt: string;
}

export type TemplateKey =
  | "resume_1pg_base"
  | "resume_2pg_base"
  | "recruiter"
  | "hiring_manager"
  | "cover_short";
