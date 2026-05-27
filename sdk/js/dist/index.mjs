// src/index.ts
import axios from "axios";
var UniversalClient = class {
  constructor(config) {
    this.config = config;
    this.accessToken = null;
    this.refreshToken = null;
    if (!config || typeof config !== "object" || !config.apiUrl) {
      throw new Error("[SDK Error] Invalid configuration provided. UniversalClient requires an object with apiUrl and projectId.");
    }
    this.http = axios.create({
      baseURL: config.apiUrl
    });
    this.http.interceptors.request.use((config2) => {
      if (this.accessToken) {
        config2.headers.Authorization = `Bearer ${this.accessToken}`;
      }
      return config2;
    });
    this.auth = {
      register: async (email, password) => {
        const { data } = await this.http.post("/auth/register", {
          project_id: this.config.projectId,
          email,
          password
        });
        this.setTokens(data);
        return data;
      },
      login: async (email, password) => {
        const { data } = await this.http.post("/auth/login", {
          project_id: this.config.projectId,
          email,
          password
        });
        this.setTokens(data);
        return data;
      },
      logout: () => {
        this.accessToken = null;
        this.refreshToken = null;
      }
    };
    this.db = {
      from: (table) => new QueryBuilder(this.http, table, this.config.projectId)
    };
    this.realtime = {
      channel: (channelName) => new RealtimeChannel(this.config.apiUrl, this.accessToken || "", channelName)
    };
    this.storage = {
      bucket: (bucketName) => new StorageBucket(this.http, bucketName, this.config.projectId)
    };
  }
  setTokens(data) {
    this.accessToken = data.access_token;
    this.refreshToken = data.refresh_token;
  }
};
var QueryBuilder = class {
  constructor(http, table, projectId) {
    this.http = http;
    this.table = table;
    this.projectId = projectId;
    this.filters = [];
    this.limitVal = null;
    this.offsetVal = null;
    this.sortVal = null;
  }
  eq(column, value) {
    this.filters.push(`${column}=eq.${value}`);
    return this;
  }
  neq(column, value) {
    this.filters.push(`${column}=neq.${value}`);
    return this;
  }
  gt(column, value) {
    this.filters.push(`${column}=gt.${value}`);
    return this;
  }
  lt(column, value) {
    this.filters.push(`${column}=lt.${value}`);
    return this;
  }
  limit(n) {
    this.limitVal = n;
    return this;
  }
  offset(n) {
    this.offsetVal = n;
    return this;
  }
  sort(column, direction = "asc") {
    this.sortVal = `${column}.${direction}`;
    return this;
  }
  async select() {
    const params = new URLSearchParams();
    this.filters.forEach((f) => params.append("filter", f));
    const finalParams = new URLSearchParams();
    this.filters.forEach((f) => {
      const [k, v] = f.split("=");
      finalParams.append(k, v);
    });
    if (this.limitVal) finalParams.append("limit", this.limitVal.toString());
    if (this.offsetVal) finalParams.append("offset", this.offsetVal.toString());
    if (this.sortVal) finalParams.append("sort", this.sortVal);
    const { data } = await this.http.get(`/api/v1/${this.table}?${finalParams.toString()}`);
    return data;
  }
};
var RealtimeChannel = class {
  constructor(url, token, channel) {
    this.url = url;
    this.token = token;
    this.channel = channel;
    this.ws = null;
    console.log(`[SDK Debug] RealtimeChannel init - url: ${url}, channel: ${channel}`);
  }
  subscribe(callback) {
    if (!this.url) {
      throw new Error("[SDK Error] RealtimeChannel url is undefined. Ensure UniversalClient is configured correctly.");
    }
    const wsUrl = this.url.replace("http", "ws") + "/realtime";
    console.log(`[SDK Debug] Connecting to: ${wsUrl}`);
    if (typeof WebSocket === "undefined") {
      console.warn("[SDK Warning] WebSocket not found in current environment. Realtime will not function in Node.js without a polyfill.");
      return;
    }
    this.ws = new WebSocket(wsUrl);
    this.ws.onopen = () => {
      this.ws?.send(JSON.stringify({
        action: "subscribe",
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
};
var StorageBucket = class {
  constructor(http, bucket, projectId) {
    this.http = http;
    this.bucket = bucket;
    this.projectId = projectId;
  }
  async upload(file) {
    const formData = new FormData();
    formData.append("file", file);
    const { data } = await this.http.post(`/storage/upload/${this.bucket}`, formData, {
      headers: { "Content-Type": "multipart/form-data" }
    });
    return data;
  }
  async sign(path) {
    const { data } = await this.http.post(`/storage/sign/${this.bucket}`, { path });
    return data.url;
  }
};
export {
  UniversalClient
};
