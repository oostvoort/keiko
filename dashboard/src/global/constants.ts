import {Provider} from "starknet";
import {getProductionUrl} from "./utils";

export const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"

export const KATANA_URL = import.meta.env.KATANA_URL ?? import.meta.env.PROD ? getProductionUrl() : 'http://localhost:5050'

export const PROVIDER = new Provider({
  rpc: {
    nodeUrl: KATANA_URL
  }
});