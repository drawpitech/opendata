import { Bounds } from "pigeon-maps";
import { match } from "ts-pattern";
import { Output, array, number, object, parse, string } from "valibot";

const EstablishmentSchema = object({
  record_id: string(),
  kind: string(),
  name: string(),
  siret: string(),
  address: string(),
  city: string(),
  postal_code: string(),
  latitude: number(),
  longitude: number(),
  inspection_date: string(),
  evaluation: string(),
});

export type Establishment = Output<typeof EstablishmentSchema>;

export async function fetchEstablishments(bounds: Bounds) {
  const apiUrl = import.meta.env.VITE_API_URL ?? "http://localhost:5050";

  const query = `ne_lat=${bounds.ne[0]}&ne_lng=${bounds.ne[1]}&sw_lat=${bounds.sw[0]}&sw_lng=${bounds.sw[1]}`;

  const response = await fetch(`${apiUrl}/api/get_near?${query}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const json = await response.json();

  return parse(array(EstablishmentSchema), json);
}

export const getEstablishmentColor = (evaluation: string) => {
  return match(evaluation)
    .with("Très satisfaisant", () => "#15803d")
    .with("Satisfaisant", () => "#65a30d")
    .with("A améliorer", () => "#d97706 ")
    .with("A corriger de manière urgente", () => "#b91c1c")
    .otherwise(() => "#737373");
};
