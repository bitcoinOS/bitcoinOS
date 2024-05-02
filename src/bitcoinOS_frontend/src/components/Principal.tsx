import { PrincipalPill } from "./PrincipalPill";
import Spinner from "./Spinner";
import { useInternetIdentity } from "ic-use-internet-identity";

export default function Principal({ principal }: { principal?: string }) {
  const { identity } = useInternetIdentity();

  if (!identity) return null;

  return (
    <div className="flex flex-col flex-wrap items-center w-full gap-5 md:gap-0 md:flex-row">
      Your principal is:
      {principal ? (
        <PrincipalPill principal={principal} />
      ) : (
        <div className="inline-block px-5 ml-3 rounded bg-zinc-600">
          <Spinner className="w-4 h-8 md:w-10 md:h-16" />
        </div>
      )}
    </div>
  );
}
