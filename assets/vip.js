import IPFS from './ipfs-core';
import OrbitDB from './orbit-db';

export class Events {
    static Ready = 'ready'; // Emitted after fully loading the local database.
    static Replicate = 'replicate'; // Emitted before replicating a part of the database with a peer.
    static ReplicationProgress = 'replicationProgress';
    static Replicated = 'replicated'; // Emitted when the database has synced with another peer.
    static Loading = 'load'; // Emitted before loading the database.
    static LoadingProgress = 'load.progress'; // Emitted while loading the local database, once for each entry.
    static Write = 'write' // Emitted after an entry was added locally to the database.
    static PeerConnected = 'peer'; // Emitted when a new peer connects via ipfs pubsub.
    static PeerExchanged = 'peer.exchanged'; // Emitted after heads have been exchanged with a peer for a specific database.
    static Closed = 'closed'; // Emitted once the database has finished closing.

    static Connected = 'connected';
    static Disconnected = 'disconnected';
    static NetworkPeersChanged = 'networkPeersChanged';
    static TotalChanged = 'totalChanged';
}

class VIP extends EventTarget {
    peers = new Peers();

    async createDatabase(name)
    {
        // Create OrbitDB database
        console.debug('OrbitDB: Creating database...');
        const options = { accessController: { write: ['*'] } }
        this.database = await this.orbitDB.create(name, 'eventlog', options);
        console.debug(`OrbitDB: Database ${this.database.id} created.`);
    }

    debug()
    {
        window.LOG='orbit*';
    }

    async openDatabase(address)
    {
        // Open database
        this.database = await this.orbitDB.open(address, { sync: true });

        // Add event listeners
        this.database.events.on(Events.Replicated, async (address) => {
            console.debug(Events.Replicated, address);
            //this.dispatchEvent(new Event(Events.Replicated));
            //await this.database.load();
            await this.refresh();
        });
        this.database.events.on(Events.Replicate, async (address) => {
            console.debug(Events.Replicate, address);

            this.dispatchEvent(new Event(Events.Replicate));
        });
        this.database.events.on(Events.ReplicationProgress, async (address, hash, entry, progress, have) => {
            console.debug(Events.ReplicationProgress, address, hash, entry, progress, have);
            await this.refresh()
        });
        this.database.events.on(Events.Loading, async (dbname) => {
            console.debug(Events.Loading, dbname);
            this.total = 0;
            this.dispatchEvent(new Event(Events.Loading));
        });
        this.database.events.on(Events.LoadingProgress, async (address, hash, entry, progress, total) => {
            console.debug(Events.LoadingProgress, address, hash, entry, progress, total);
            // this.progress = progress / total;
            // this.total = total;
            this.dispatchEvent(new Event(Events.LoadingProgress))
        });
        this.database.events.on(Events.Ready, async (dbname, heads) => {
            console.debug(Events.Ready, dbname, heads);
            this.dispatchEvent(new Event(Events.Ready));
            await this.refresh();
        });
        this.database.events.on(Events.Write, async (address, entry, heads) => {
            console.debug(Events.Write, address, entry, heads);
            await this.refresh()
        });
        this.database.events.on(Events.Closed, async (dbname) => {
            console.debug(Events.Closed, dbname);
            await this.refresh()
        });
        this.database.events.on(Events.PeerConnected, async (peer) => {
            console.debug(Events.Write, peer);

            await this.refresh();

            if (this.peers.database >= 0)
                this.dispatchEvent(new Event(Events.Connected));
            else
                this.dispatchEvent(new Event(Events.Disconnected));
        });
        this.database.events.on(Events.PeerExchanged, async (peer, address, heads)=> {
            console.debug(Events.PeerExchanged, peer, address, heads);

            await this.refresh();

            if (this.peers.database >= 0)
                this.dispatchEvent(new Event(Events.Connected));
            else
                this.dispatchEvent(new Event(Events.Disconnected));
        });
    }

    async init() {
        // Create IPFS node
        console.debug('IPFS: Creating node in browser...');
        this.ipfsNode = await IPFS.create({
            config: {
                Addresses: {
                    Swarm: [
                        '/dns4/wrtc-star1.par.dwebops.pub/tcp/443/wss/p2p-webrtc-star/',
                        '/dns4/wrtc-star2.sjc.dwebops.pub/tcp/443/wss/p2p-webrtc-star/',
                        //'/dns4/webrtc-star.discovery.libp2p.io/tcp/443/wss/p2p-webrtc-star/',
                    ]
                },
            },
            preload: {
                enabled: false
            },
            relay: { enabled: true, hop: { enabled: true, active: true } },
            start: true,
        });

        // console.debug(this.ipfsNode);
        console.debug(`IPFS: Started.`);
        const identity = await this.ipfsNode.id();
        console.debug(`IPFS: Node ${identity.id} created.`);
        console.log(identity.addresses);

       // await
       // this.ipfsNode.swarm.connect('/dns4/wrtc-star1.par.dwebops.pub/tcp/443/wss/p2p-webrtc-star/p2p/12D3KooWNvnTi5tssk7TcqsnoNe6ktBgWuNkNu4PRrDdi39Gc7zm')

        // for await (const res of this.ipfsNode.ping('12D3KooWC22HxaR4kcf3TraNJuVCJxJ6QjNoUWLLiNuYwP2xaBN6')) {
        //     if (res.time) {
        //         console.log(`Pong received: time=${res.time} ms`)
        //     } else {
        //         console.log(res.text)
        //     }
        // }

        // let bootstrapAddresses = await this.ipfsNode.bootstrap.list()
        // console.log(bootstrapAddresses.Peers)
        //
        // const swarmPeers = await this.ipfsNode.swarm.peers()
        // console.log(swarmPeers)

        await this.refresh();

        // Create OrbitDB instance
        console.debug('OrbitDB: Creating instance...');
        this.orbitDB = await OrbitDB.createInstance(this.ipfsNode)
        console.debug(`OrbitDB: Instance ${this.orbitDB.identity.id} created.`);
        //console.log(this.orbitDB);
    }

    async connect(databaseAddress) {
        console.debug(`Opening database ${databaseAddress}...`);
        await this.openDatabase(databaseAddress);

        // Load existing database content into memory
        console.debug(`Loading database ${databaseAddress} content...`);
        await this.database.load();
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
        let cid = await this.database.add({ address: address });
        console.debug(`OrbitDB: ${address} added to database with cid of ${cid}`);

        this.log(`Address ${address} added to VIP list.`);

        this.total += 1;
        return cid;
    };

    async check(address)
    {
        for (let e of this.database.iterator({ limit: -1 }))
            if (e.payload.value.address === address)
                return true;

        return false;
    }

    // async entries()
    // {
    //     return this.database.iterator({ limit: -1 }).collect().map((e) => e.payload.value);
    // }

    // async networkPeers() {
    //     return await this.ipfsNode.swarm.peers();
    // }
    //
    // async databasePeers() {
    //     return await this.ipfsNode.pubsub.peers(this.database.address.toString());
    // }

    async refresh(){
        // Refresh network peers
        let peers = await this.ipfsNode.swarm.peers();
        if (peers.length !== this.peers.network)
        {
            this.peers.network = peers.length;
            this.dispatchEvent(new Event(Events.NetworkPeersChanged));
        }

        if (this.database !== undefined && this.database.address !== undefined){

            // Refresh database peers
            peers = await this.ipfsNode.pubsub.peers(this.database.address.toString());
            if (peers.length !== this.peers.database)
            {
                this.peers.database = peers.length;
                this.dispatchEvent(new Event(Events.PeerConnected));
            }

            // Refresh total
            console.debug(this.database._replicationStatus.progress, this.database._oplog.length);
            const total = Math.max(this.database._replicationStatus.progress, this.database._oplog.length);
            if (total !== this.total)
            {
                this.total = total;
                this.dispatchEvent(new Event(Events.TotalChanged));
            }
        }
    }
}

class Peers {
    database = 0;
    network = 0;
}

export default new VIP()