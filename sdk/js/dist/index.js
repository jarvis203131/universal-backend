"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/index.ts
var index_exports = {};
__export(index_exports, {
  UniversalClient: () => UniversalClient
});
module.exports = __toCommonJS(index_exports);
var import_axios = __toESM(require("axios"));
var UniversalClient = class {
  constructor(config) {
    this.config = config;
    this.accessToken = null;
    this.refreshToken = null;
    // --- Auth Module ---
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
    // --- Database Module (Fluent Query Builder) ---
    this.db = {
      from: (table) => new QueryBuilder(this.http, table, this.config.projectId)
    };
    // --- Realtime Module ---
    this.realtime = {
      channel: (channelName) => new RealtimeChannel(this.config.apiUrl, this.accessToken || "", channelName)
    };
    // --- Storage Module ---
    this.storage = {
      bucket: (bucketName) => new StorageBucket(this.http, bucketName, this.config.projectId)
    };
    this.http = import_axios.default.create({
      baseURL: config.apiUrl
    });
    this.http.interceptors.request.use((config2) => {
      if (this.accessToken) {
        config2.headers.Authorization = `Bearer ${this.accessToken}`;
      }
      return config2;
    });
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
  }
  subscribe(callback) {
    const wsUrl = this.url.replace("http", "ws") + "/realtime";
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
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  UniversalClient
});
