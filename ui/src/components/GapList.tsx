interface Props {
  gaps: string[];
}

export function GapList({ gaps }: Props) {
  return (
    <section className="card stack-sm">
      <h3>Gaps</h3>
      <p className="subtle">
        Gaps are framed as role requirements to learn or confirm, not as invented experience claims.
      </p>
      {gaps.length === 0 ? (
        <p className="subtle">No critical gaps detected.</p>
      ) : (
        <ul>
          {gaps.map((gap) => (
            <li key={gap}>{gap}</li>
          ))}
        </ul>
      )}
    </section>
  );
}
