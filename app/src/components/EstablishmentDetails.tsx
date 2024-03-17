import { Establishment, getEstablishmentColor } from "../hooks/api";

export type EstablishmentDetailsProps = {
  establishment?: Establishment;
};

export function EstablishmentDetails({
  establishment,
}: EstablishmentDetailsProps) {
  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-center gap-2 text-gray-800">
        <span className="icon-[lucide--store]" />
        <span className="font-medium">Informations sur le restaurant</span>
      </div>

      {establishment ? (
        <div className="flex flex-col gap-1">
          <div className="flex items-center gap-1">
            <div className="font-medium">Nom:</div>
            <div className="text-gray-800">{establishment.name}</div>
          </div>

          <div className="flex items-center gap-1">
            <div className="font-medium">Addresse:</div>
            <div className="text-gray-800">
              {establishment.address} {establishment.postal_code}{" "}
              {establishment.city}
            </div>
          </div>

          <div className="flex items-center gap-1">
            <div className="font-medium">Évaluation:</div>
            <div
              className="rounded-full px-2 py-0.5"
              style={{
                backgroundColor: getEstablishmentColor(
                  establishment.evaluation,
                ),
              }}
            >
              <div className="text-sm font-medium text-white ">
                {establishment.evaluation}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-1">
            <div className="font-medium">Date de l'inspection:</div>
            <div className="text-gray-800">
              {new Date(establishment.inspection_date).toLocaleDateString()}
            </div>
          </div>

          <div className="flex items-center gap-1">
            <div className="font-medium">Siret:</div>
            <div className="text-gray-800">{establishment.siret}</div>
          </div>
        </div>
      ) : (
        <div className="text-gray-700">
          Sélectionnez un restaurant sur la carte pour afficher ses
          informations.
        </div>
      )}
    </div>
  );
}
