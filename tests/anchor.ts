import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import type { AgendaSolana } from "../target/types/agenda_solana";

describe("agenda-solana", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AgendaSolana as anchor.Program<AgendaSolana>;
  

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AgendaSolana;

  it("Crear tarea", async () => {

    const titulo = "Estudiar Solana";

    const [tareaPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tarea"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(titulo)
      ],
      program.programId
    );

    await program.methods
      .crearTarea(
        titulo,
        "Practicar Rust",
        "10 marzo"
      )
      .accounts({
        tarea: tareaPda,
        usuario: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Tarea creada");
  });

});