import axios, { AxiosInstance } from 'axios';

export interface SDKConfig {
  apiUrl: string;
  projectId: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user_id: string;
}

export class UniversalClient {
  private http: AxiosInstance;
  private accessToken: string | null = null;
  private refreshToken: string | null = null;

  constructor(private config: SDKConfig) {
    this.http = axios.create({
      baseURL: config.apiUrl,
    });

    this.http.interceptors.request.use((config) => {
      if (this.accessToken) {
        config.headers.Authorization = `Bearer ${this.accessToken}`;
      }
      return config;
    });
  }

  // --- Auth Module ---
  public auth = {
    register: async (email: string, password: string) => {
      const { data } = await this.http.post('/auth/register', {
        project_id: this.config.projectId,
        email,
        password,
      });
      this.setTokens(data);
      return data;
    },
    login: async (email: string, password: string) => {
      const { data } = await this.http.post('/auth/login', {
        project_id: this.config.projectId,
        email,
        password,
      });
      this.setTokens(data);
      return data;
    },
    logout: () => {
      this.accessToken = null;
      this.refreshToken = null;
    }
  };

  // --- Database Module (Fluent Query Builder) ---
  public db = {
    from: (table: string) => new QueryBuilder(this.http, table, this.config.projectId),
  };

  // --- Realtime Module ---
  public realtime = {
    channel: (channelName: string) => new RealtimeChannel(this.config.apiUrl, this.accessToken || '', channelName),
  };

  // --- Storage Module ---
  public storage = {
    bucket: (bucketName: string) => new StorageBucket(this.http, bucketName, this.config.projectId),
  };

  private setTokens(data: AuthResponse) {
    this.accessToken = data.access_token;
    this.refreshToken = data.refresh_token;
  }
}

class QueryBuilder {
  private filters: string[] = [];
  private limitVal: number | null = null;
  private offsetVal: number | null = null;
  private sortVal: string | null = null;

  constructor(private http: AxiosInstance, private table: string, private projectId: string) {}

  eq(column: string, value: any): this {
    this.filters.push(`${column}=eq.${value}`);
    return this;
  }
  neq(column: string, value: any): this {
    this.filters.push(`${column}=neq.${value}`);
    return this;
  }
  gt(column: string, value: any): this {
    this.filters.push(`${column}=gt.${value}`);
    return this;
  }
  lt(column: string, value: any): this {
    this.filters.push(`${column}=lt.${value}`);
    return this;
  }
  limit(n: number): this {
    this.limitVal = n;
    return this;
  }
  offset(n: number): this {
    this.offsetVal = n;
    return this;
  }
  sort(column: string, direction: 'asc' | 'desc' = 'asc'): this {
    this.sortVal = `${column}.${direction}`;
    return this;
  }

  async select() {
    const params = new URLSearchParams();
    this.filters.forEach(f => params.append('filter', f)); // Note: Backend expects query params for filters
    // Actually, based on Phase 2, it's `?column=op.value`. Let's adjust.
    
    // Correcting to Phase 2 Spec:
    const finalParams = new URLSearchParams();
    this.filters.forEach(f => {
        const [k, v] = f.split('=');
        finalParams.append(k, v);
    });

    if (this.limitVal) finalParams.append('limit', this.limitVal.toString());
    if (this.offsetVal) finalParams.append('offset', this.offsetVal.toString());
    if (this.sortVal) finalParams.append('sort', this.sortVal);

    const { data } = await this.http.get(`/api/v1/${this.table}?${finalParams.toString()}`);
    return data;
  }
}

class RealtimeChannel {
  private ws: WebSocket | null = null;

  constructor(private url: string, private token: string, private channel: string) {}

  subscribe(callback: (payload: any) => void) {
    const wsUrl = this.url.replace('http', 'ws') + '/realtime';
    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      this.ws?.send(JSON.stringify({
        action: 'subscribe',
        channel: this.channel,
        token: this.token
      }));
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      callback(data);
    };
  }

  unsubscribe() {
    this.ws?.close();
  }
}

class StorageBucket {
  constructor(private http: AxiosInstance, private bucket: string, private projectId: string) {}

  async upload(file: File | Blob) {
    const formData = new FormData();
    formData.append('file', file);
    const { data } = await this.http.post(`/storage/upload/${this.bucket}`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    });
    return data;
  }

  async sign(path: string) {
    const { data } = await this.http.post(`/storage/sign/${this.bucket}`, { path });
    return data.url;
  }
}
