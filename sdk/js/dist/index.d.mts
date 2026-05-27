import { AxiosInstance } from 'axios';

interface SDKConfig {
    apiUrl: string;
    projectId: string;
}
interface AuthResponse {
    access_token: string;
    refresh_token: string;
    user_id: string;
}
declare class UniversalClient {
    private config;
    private http;
    private accessToken;
    private refreshToken;
    auth: {
        register: (email: string, password: string) => Promise<any>;
        login: (email: string, password: string) => Promise<any>;
        logout: () => void;
    };
    db: {
        from: (table: string) => QueryBuilder;
    };
    realtime: {
        channel: (channelName: string) => RealtimeChannel;
    };
    storage: {
        bucket: (bucketName: string) => StorageBucket;
    };
    constructor(config: SDKConfig);
    private setTokens;
}
declare class QueryBuilder {
    private http;
    private table;
    private projectId;
    private filters;
    private limitVal;
    private offsetVal;
    private sortVal;
    constructor(http: AxiosInstance, table: string, projectId: string);
    eq(column: string, value: any): this;
    neq(column: string, value: any): this;
    gt(column: string, value: any): this;
    lt(column: string, value: any): this;
    limit(n: number): this;
    offset(n: number): this;
    sort(column: string, direction?: 'asc' | 'desc'): this;
    select(): Promise<any>;
}
declare class RealtimeChannel {
    private url;
    private token;
    private channel;
    private ws;
    constructor(url: string, token: string, channel: string);
    subscribe(callback: (payload: any) => void): void;
    unsubscribe(): void;
}
declare class StorageBucket {
    private http;
    private bucket;
    private projectId;
    constructor(http: AxiosInstance, bucket: string, projectId: string);
    upload(file: File | Blob): Promise<any>;
    sign(path: string): Promise<any>;
}

export { type AuthResponse, type SDKConfig, UniversalClient };
