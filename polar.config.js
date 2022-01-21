const accounts = [
  {
    name: 'a',
    address: 'secret1q2ewmd687qldpvp4vncrjtdc6ddw439qlv807s',
    mnemonic: 'echo strong slender door alley awful next evolve flame item direct woman swallow gentle stool stock banner enjoy buddy orchard throw coconut build mix'
  },
  {
    name: 'b',
    address: 'secret1y98fdsl6vv3qdrwqfc74p45su5y8m4vqqgdfkw',
    mnemonic: 'yellow inflict open stage portion memory obtain situate assault vicious crack hospital patch sport congress peasant lizard couch render obtain crew exercise bulk fire'
  }
];

module.exports = {
  defaultNetwork: 'development',
  networks: {
    development: {
      endpoint: 'tcp://0.0.0.0:26657',
      nodeId: '115aa0a629f5d70dd1d464bc7e42799e00f4edae',
      chainId: 'secretdev-1',
      trustNode: true,
      keyringBackend: 'test',
      accounts: accounts,
      types: {}
    },
    default: {
      endpoint: 'http://0.0.0.0:1337',
      nodeId: '115aa0a629f5d70dd1d464bc7e42799e00f4edae',
      chainId: 'secretdev-1',
      trustNode: true,
      keyringBackend: 'test',
      accounts: accounts,

      types: {}
    },
    // Supernova Testnet
    testnet: {
      endpoint: 'http://bootstrap.supernova.enigma.co:1317',
      chainId: 'supernova-2',
      trustNode: true,
      keyringBackend: 'test',
      accounts: accounts,
      types: {},
      fees: {
        upload: {
          amount: [{ amount: "500000", denom: "uscrt" }],
          gas: "2000000",
        },
        init: {
          amount: [{ amount: "125000", denom: "uscrt" }],
          gas: "500000",
        },
      }
    }
  },
  mocha: {
    timeout: 60000
  }
};