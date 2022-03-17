import IPFS from './ipfs-core';
import OrbitDB from './orbit-db';

class VIP {
    constructor (Ipfs, OrbitDB) {
        this.Ipfs = Ipfs
        this.OrbitDB = OrbitDB
    }

    async start() {
        this.log('Creating VIP list...');

        // Create IPFS node
        console.debug('IPFS: Creating node in browser...');
        this.ipfsNode = await IPFS.create();
        //console.log(this.ipfsNode);
        const identity = await this.ipfsNode.id();
        const nodeId = identity.id;
        console.debug(`IPFS: Node ${identity.id} created.`);

        // Create OrbitDB instance
        console.debug('OrbitDB: Creating instance...');
        this.orbitDB = await OrbitDB.createInstance(this.ipfsNode)
        console.debug(`OrbitDB: Instance ${this.orbitDB.identity.id} created.`);
        //console.log(this.orbitDB);

        // Create OrbitDB database
        console.debug('OrbitDB: Creating database...');
        this.defaultOptions = { accessController: { write: [this.orbitDB.identity.id] } }
        const docStoreOptions = {
                 ...this.defaultOptions,
                 indexBy: 'hash',
               };
        this.vipList = await this.orbitDB.feed('vip-list', docStoreOptions)
        console.debug(`OrbitDB: Database ${this.vipList.id} created.`);

        // Load database content
        this.vipList.events.on('load.progress', (address, hash, entry, progress, total) => {
            //console.log('Loading:', address, hash, entry, progress, total);
        })
        await this.vipList.load();

        this.log('VIP list ready.');
    }

    log(message){
        console.log(`MFHQ: ${message}`);
    }

    async register(address) {

        this.log(`Adding address ${address} to VIP list...`);

        if (await this.check(address)) {
            this.log(`Address ${address} already added.`);
            return;
        }

        console.debug(`OrbitDB: Adding ${address} to database...`);
        let cid = await this.vipList.add({ address: address });
        console.debug(`OrbitDB: ${address} added to database with cid of ${cid}`);

        this.log(`Address ${address} added to VIP list.`);
        return cid;
    };

    async check(address)
    {
        for (let e of this.vipList.iterator({ limit: -1 }))
            if (e.payload.value.address === address)
                return true;

        return false;
    }

    async entries()
    {
        return this.vipList.iterator({ limit: -1 }).collect().map((e) => e.payload.value);
    }
}

export default new VIP()