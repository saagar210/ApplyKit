import type { FitBreakdown } from "../lib/types";

interface Props {
  fit?: FitBreakdown;
  track?: string;
}

export function FitScoreCard({ fit, track }: Props) {
  return (
    <section className="card stack-sm">
      <h3>Fit Score</h3>
      <div className="fit-total">{fit?.total ?? "--"}</div>
      <p className="subtle">Track: {track ?? "Not generated yet"}</p>
      {fit ? (
        <>
          <ul>
            <li>Role match: {fit.roleMatch}</li>
            <li>Stack match: {fit.stackMatch}</li>
            <li>Scale match: {fit.scaleMatch}</li>
            <li>Rigor match: {fit.rigorMatch}</li>
            <li>Signal boost: {fit.signalBoost}</li>
          </ul>
          <div>
            <strong>Why match</strong>
            <ul>
              {fit.whyMatch.map((line) => (
                <li key={line}>{line}</li>
              ))}
            </ul>
          </div>
          <div>
            <strong>Gaps</strong>
            {fit.gaps.length === 0 ? <p className="subtle">No critical gaps detected.</p> : null}
            <ul>
              {fit.gaps.map((line) => (
                <li key={line}>{line}</li>
              ))}
            </ul>
          </div>
        </>
      ) : (
        <p className="subtle">Fit breakdown appears after generation.</p>
      )}
    </section>
  );
}
