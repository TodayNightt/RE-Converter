/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import { AppContextProvider } from "./appContext";
render(
  () => (
    <AppContextProvider>
      <App />
    </AppContextProvider>
  ),
  document.getElementById("root") as HTMLElement
);
