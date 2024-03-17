import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { EstablishmentMap } from "./components/EstablishmentMap";

const queryClient = new QueryClient();

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="h-screen w-screen">
        <EstablishmentMap />
      </div>
    </QueryClientProvider>
  );
}
