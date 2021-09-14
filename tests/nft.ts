import * as anchor from '@project-serum/anchor';
//import {Provider } from anchor;
import {  
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Keypair,
  TransactionInstruction} from "@solana/web3.js";
import {assert, expect} from "chai";
import BN from 'bn.js';

describe('nft', () => {

  // Configure the client to use the local cluster.
  const provider : anchor.Provider = anchor.Provider.local(); 
  anchor.setProvider(provider);
  const nftAccount: Keypair  = anchor.web3.Keypair.generate();


  it('Is initialized!', async () => {
    const timestamps = Math.round(new Date().getTime() / 1000) + 2000;
    const solPriceStr = '1';
    const lamports = parseInt(solPriceStr) * LAMPORTS_PER_SOL;

    //const nftAccount: Keypair  = anchor.web3.Keypair.generate();

    const program = anchor.workspace.Nft;
    const tx = await program.rpc.initialize(timestamps.toString(), {
      uuid : "nft_contract",
      price : new anchor.BN(lamports),
      itemsAvailable : new anchor.BN(100),
      goLiveDate : null
    }, {
      accounts : {
        nftAccount : nftAccount.publicKey,
        admin : provider.wallet.publicKey,
        systemProgram : SystemProgram.programId
      },
      signers : [nftAccount]
    });

    assert.isString("tr_test", tx);
    const nftAccountData:NftAccountData = await program.account.nftAccountData.fetch(nftAccount.publicKey);
    assert.ok(nftAccountData.admin.equals(provider.wallet.publicKey));
    console.log(nftAccountData);
    //done();
  });

  it("should update config data", async() => {
    const solPriceStr = '1';
    const lamports = parseInt(solPriceStr) * LAMPORTS_PER_SOL;


    const program = anchor.workspace.Nft;
    const tx = await program.rpc.updateNftConfig(
      {
      uuid : "nft_contract",
      price : new anchor.BN(lamports),
      itemsAvailable : new anchor.BN(100),
      goLiveDate : null
    },
     {
      accounts : {
        nftAccount : nftAccount.publicKey,
        admin : provider.wallet.publicKey,
        systemProgram : SystemProgram.programId
      },
     // signers : [nftAccount]
    });

    assert.isString("tr_test", tx);
  });
});


interface NftAccountData {
  admin : PublicKey,
  buyers : Array<PublicKey>,
  nftConfigData : ConfigData
 
}

interface ConfigData {
  uuid : string,
  price : BN,
  itemsAvailable : number,
  goLiveDate : any
}
