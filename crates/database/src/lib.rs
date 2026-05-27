use sea_query::{Query, PostgresQueryBuilder, Expr, Condition, Order, Value, SimpleExpr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOp {
    Eq,
    Neq,
    Gte,
    Lte,
    Gt,
    Lt,
}

impl FilterOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "eq" => Some(Self::Eq),
            "neq" => Some(Self::Neq),
            "gte" => Some(Self::Gte),
            "lte" => Some(Self::Lte),
            "gt" => Some(Self::Gt),
            "lt" => Some(Self::Lt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicQuery {
    pub filters: HashMap<String, String>, // "column=op.value"
    pub sort: Option<String>,             // "column.direction"
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub struct QueryEngine;

impl QueryEngine {
    pub fn build_select(
        table_name: &str,
        dq: &DynamicQuery,
        project_id: Uuid,
    ) -> (String, Vec<Value>) {
        let mut query = Query::select();
        query.from(sea_query::Alias::new(table_name))
            .columns(vec![sea_query::Asterisk]);

        let mut condition = Condition::all()
            .add(Expr::col(sea_query::Alias::new("project_id")).eq(project_id.to_string()));

        for (col, val_str) in &dq.filters {
            if let Some((op_str, value)) = val_str.split_once('.') {
                if let Some(op) = FilterOp::from_str(op_str) {
                    let col_expr = Expr::col(sea_query::Alias::new(col));
                    match op {
                        FilterOp::Eq => { condition = condition.add(col_expr.eq(value)); }
                        FilterOp::Neq => { condition = condition.add(col_expr.eq(value).not()); }
                        FilterOp::Gte => { condition = condition.add(col_expr.gte(value)); }
                        FilterOp::Lte => { condition = condition.add(col_expr.lte(value)); }
                        FilterOp::Gt => { condition = condition.add(col_expr.gt(value)); }
                        FilterOp::Lt => { condition = condition.add(col_expr.lt(value)); }
                    }
                }
            } else {
                condition = condition.add(Expr::col(sea_query::Alias::new(col)).eq(val_str));
            }
        }

        query.cond_where(condition);

        if let Some(sort_str) = &dq.sort {
            if let Some((col, dir)) = sort_str.split_once('.') {
                let order = match dir {
                    "desc" => Order::Desc,
                    _ => Order::Asc,
                };
                query.order_by(sea_query::Alias::new(col), order);
            }
        }

        if let Some(limit) = dq.limit {
            query.limit(limit);
        }

        if let Some(offset) = dq.offset {
            query.offset(offset);
        }

        let (sql, values) = query.build(PostgresQueryBuilder);
        (sql, values.into_iter().collect())
    }

    pub fn build_insert(
        table_name: &str,
        data: HashMap<String, serde_json::Value>,
        project_id: Uuid,
    ) -> (String, Vec<Value>) {
        let mut query = Query::insert();
        query.into_table(sea_query::Alias::new(table_name));

        let mut cols = Vec::new();
        let mut vals = Vec::new();

        cols.push(sea_query::Alias::new("project_id"));
        vals.push(SimpleExpr::Value(Value::from(project_id.to_string())));

        for (k, v) in data {
            cols.push(sea_query::Alias::new(k));
            let val = match v {
                serde_json::Value::String(s) => Value::from(s),
                serde_json::Value::Number(n) => Value::from(n.to_string()),
                serde_json::Value::Bool(b) => Value::from(b),
                _ => Value::from("NULL"),
            };
            vals.push(SimpleExpr::Value(val));
        }

        query.columns(cols).values(vals);
        let (sql, values) = query.build(PostgresQueryBuilder);
        (sql, values.into_iter().collect())
    }

    pub fn build_update(
        table_name: &str,
        id: Uuid,
        data: HashMap<String, serde_json::Value>,
        project_id: Uuid,
    ) -> (String, Vec<Value>) {
        let mut query = Query::update();
        query.table(sea_query::Alias::new(table_name));

        let mut values = Vec::new();
        for (k, v) in data {
            let val = match v {
                serde_json::Value::String(s) => Value::from(s),
                serde_json::Value::Number(n) => Value::from(n.to_string()),
                serde_json::Value::Bool(b) => Value::from(b),
                _ => Value::from("NULL"),
            };
            values.push((sea_query::Alias::new(k), SimpleExpr::Value(val)));
        }

        query.values(values);

        query.cond_where(
            Condition::all()
                .add(Expr::col(sea_query::Alias::new("id")).eq(id.to_string()))
                .add(Expr::col(sea_query::Alias::new("project_id")).eq(project_id.to_string())),
        );

        let (sql, values) = query.build(PostgresQueryBuilder);
        (sql, values.into_iter().collect())
    }

    pub fn build_delete(
        table_name: &str,
        id: Uuid,
        project_id: Uuid,
    ) -> (String, Vec<Value>) {
        let mut query = Query::delete();
        query.from_table(sea_query::Alias::new(table_name))
            .cond_where(
                Condition::all()
                    .add(Expr::col(sea_query::Alias::new("id")).eq(id.to_string()))
                    .add(Expr::col(sea_query::Alias::new("project_id")).eq(project_id.to_string())),
            );

        let (sql, values) = query.build(PostgresQueryBuilder);
        (sql, values.into_iter().collect())
    }
}
