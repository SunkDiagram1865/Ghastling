use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct JavaVersion {
    pub parsed_version: u32,
    pub version: String,
    pub architecture: String,
    pub path: String,
    pub distribution: Option<String>,
}

impl JavaVersion {
    pub async fn get(
        major_version: u32,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<Option<JavaVersion>> {
        let version = major_version as i32;

        let res = sqlx::query!(
            "
            SELECT
                full_version, architecture, path, distribution
            FROM java_versions
            WHERE major_version = $1
            ",
            version
        )
        .fetch_optional(exec)
        .await?;

        Ok(res.map(|x| JavaVersion {
            parsed_version: major_version,
            version: x.full_version,
            architecture: x.architecture,
            path: x.path,
            distribution: x.distribution,
        }))
    }

    pub async fn get_all(
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<Vec<Self>> {
        let rows = sqlx::query!(
            r#"SELECT major_version, full_version, architecture, path, distribution as "distribution?: String" FROM java_versions"#
        )
        .fetch_all(exec)
        .await?;

        Ok(rows
            .into_iter()
            .map(|x| JavaVersion {
                parsed_version: x.major_version as u32,
                version: x.full_version,
                architecture: x.architecture,
                path: x.path,
                distribution: x.distribution,
            })
            .collect())
    }

    pub async fn upsert(
        &self,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<()> {
        let major_version = self.parsed_version as i32;

        sqlx::query!(
            "
            INSERT INTO java_versions (major_version, full_version, architecture, path, distribution)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (path) DO UPDATE SET
                major_version = $1,
                full_version = $2,
                architecture = $3,
                distribution = $5
            ",
            major_version,
            self.version,
            self.architecture,
            self.path,
            self.distribution,
        )
            .execute(exec)
            .await?;

        Ok(())
    }
    pub async fn delete(
        path: &str,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<()> {
        sqlx::query!("DELETE FROM java_versions WHERE path = $1", path)
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn remove(
        major_version: u32,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<()> {
        let version = major_version as i32;
        sqlx::query("DELETE FROM java_versions WHERE major_version = $1")
            .bind(version)
            .execute(exec)
            .await?;
        Ok(())
    }
}
