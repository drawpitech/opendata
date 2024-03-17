import { ReactNode } from "react";

export type InfoOverlayProps = {
  children?: ReactNode;
};

export function InfoOverlay({ children }: InfoOverlayProps) {
  return (
    <div className="absolute right-0 top-0 mx-10 my-10 w-[500px]">
      <div className="rounded-md bg-white p-4 shadow-xl">
        <div className="mb-4">
          <h1 className="text-lg font-semibold">Palachias</h1>
          <p className="text-gray-800">
            Consultez les contrôles sanitaires de sûreté alimentaires réalisés
            dans les restaurants près de chez vous.
          </p>
        </div>

        {children}
      </div>
    </div>
  );
}
