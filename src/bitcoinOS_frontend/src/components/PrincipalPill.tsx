export function PrincipalPill({ principal }: { principal: string }) {
  return (
    <div className="inline-block h-8 px-5 rounded md:my-5 md:ml-3 md:h-16 bg-zinc-600">
      {principal.slice(0, 5)}…{principal.slice(-3)}
    </div>
  );
}
