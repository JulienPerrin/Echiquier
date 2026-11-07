import "gchessboard";
import type { GChessBoardElement } from "gchessboard";
import { invoke } from "@tauri-apps/api/core";

const board = document.querySelector<GChessBoardElement>("#board");
const apiStatus = document.querySelector<HTMLParagraphElement>("#api-status");

async function fetchHelloWorldFen(): Promise<string> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string>("hello_world");
  }
  const response = await fetch("/api/hello_world");
  return (await response.text()).trim();
}

async function initApi(): Promise<void> {
  const fen = await fetchHelloWorldFen();
  if (board) {
    board.fen = fen;
  }
  if (apiStatus) {
    apiStatus.textContent = fen;
  }
}

void initApi();
