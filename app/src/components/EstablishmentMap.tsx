import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Bounds, Map, Marker, ZoomControl } from "pigeon-maps";
import { osm } from "pigeon-maps/providers";
import { useState } from "react";
import {
  Establishment,
  fetchEstablishments,
  getEstablishmentColor,
} from "../hooks/api";
import { InfoOverlay } from "./InfoOverlay";
import { EstablishmentDetails } from "./EstablishmentDetails";

export function EstablishmentMap() {
  const [bounds, setBounds] = useState<Bounds>({ ne: [0, 0], sw: [0, 0] });
  const [establishment, setEstablishment] = useState<Establishment>();

  const { data: establishments } = useQuery({
    queryKey: ["establishments", bounds],
    queryFn: () => fetchEstablishments(bounds),
    placeholderData: keepPreviousData,
  });

  return (
    <>
      <Map
        onBoundsChanged={({ bounds }) => setBounds(bounds)}
        defaultCenter={[44.84, -0.58]}
        defaultZoom={13}
        provider={osm}
      >
        <ZoomControl />

        {establishments?.map((establishment) => (
          <Marker
            key={establishment.record_id}
            anchor={[establishment.latitude, establishment.longitude]}
            color={getEstablishmentColor(establishment.evaluation)}
            onClick={() => setEstablishment(establishment)}
          />
        ))}
      </Map>

      <InfoOverlay>
        <EstablishmentDetails establishment={establishment} />
      </InfoOverlay>
    </>
  );
}
