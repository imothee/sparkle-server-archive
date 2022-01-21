use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::env;
use chrono::{Datelike, DateTime, Duration, NaiveDate, TimeZone, Utc, Weekday};

use sparkle_server::models::app::App;
use sparkle_server::models::system_profiles::SystemProfile;
use sparkle_server::models::metric::{Metric, NewMetric};
use sparkle_server::models::schema::metrics::app_id;

#[derive(Debug)]
struct Counter {
  value: String,
  count: i32,
}

#[derive(Default, Debug)]
struct Stats {
  app_versions: Vec<Counter>,
  cpu64bits: Vec<Counter>,
  ncpus: Vec<Counter>,
  cpu_freq_mhzs: Vec<Counter>,
  cputypes: Vec<Counter>,
  cpusubtypes: Vec<Counter>,
  models: Vec<Counter>,
  ram_mbs: Vec<Counter>,
  os_versions: Vec<Counter>,
  langs: Vec<Counter>,
}

fn main() {
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  let conn = pool.get().expect("couldn't get db connection from pool");
  let apps = App::all(&conn).expect("couldn't get apps");

  let args: Vec<String> = env::args().collect();
  let offset = args.get(1).map(|f| f.parse::<i32>().unwrap_or_else(|e| { 0 })).unwrap();

  let days_from_sunday = Utc::now().weekday().num_days_from_sunday();
  let time = match offset {
    0 => Utc::now().sub(Duration::days(days_from_sunday as i64)),
    int => Utc::now().sub(Duration::days(days_from_sunday as i64)).sub(Duration::weeks(int as i64)),
  };
  let local_time = NaiveDate::from_ymd(
    time.naive_local().year(),
    time.naive_local().month(),
    time.naive_local().day(),
  ).and_hms(0, 0, 0);
  let start = time.timezone().from_local_datetime(&local_time).unwrap();
  let end = start.clone().add(Duration::weeks(1));

  for app in apps {
    let profiles = SystemProfile::by_app_id_between(&app.id, &start, &end,&conn)
      .expect("couldn't get profiles");

    let mut stats = Stats::default();
    let numProfiles = (&profiles.len()).clone() as i32;

    for profile in profiles {
      if let Some(app_version) = profile.app_version {
        if let Some(app_versions) = stats.app_versions
          .iter_mut()
          .find(|f| f.value == app_version)
        {
          app_versions.count += 1;
        } else {
          stats.app_versions.push(Counter {
            value: app_version.into(),
            count: 1,
          });
        }
      }

      if let Some(cpu_freq_mhz) = profile.cpu_freq_mhz {
        if let Some(cpu_freq_mhzs) = stats.cpu_freq_mhzs
          .iter_mut()
          .find(|f| f.value == cpu_freq_mhz)
        {
          cpu_freq_mhzs.count += 1;
        } else {
          stats.cpu_freq_mhzs.push(Counter {
            value: cpu_freq_mhz.into(),
            count: 1,
          });
        }
      }

      if let Some(cputype) = profile.cputype {
        if let Some(cputypes) = stats.cputypes
          .iter_mut()
          .find(|f| f.value == cputype)
        {
          cputypes.count += 1;
        } else {
          stats.cputypes.push(Counter {
            value: cputype.into(),
            count: 1,
          });
        }
      }

      if let Some(cpusubtype) = profile.cpusubtype {
        if let Some(cpusubtypes) = stats.cpusubtypes
          .iter_mut()
          .find(|f| f.value == cpusubtype)
        {
          cpusubtypes.count += 1;
        } else {
          stats.cpusubtypes.push(Counter {
            value: cpusubtype.into(),
            count: 1,
          });
        }
      }

      if let Some(model) = profile.model {
        if let Some(models) = stats.models
          .iter_mut()
          .find(|f| f.value == model)
        {
          models.count += 1;
        } else {
          stats.models.push(Counter {
            value: model.into(),
            count: 1,
          });
        }
      }

      if let Some(ram_mb) = profile.ram_mb {
        if let Some(ram_mbs) = stats.ram_mbs
          .iter_mut()
          .find(|f| f.value == ram_mb)
        {
          ram_mbs.count += 1;
        } else {
          stats.ram_mbs.push(Counter {
            value: ram_mb.into(),
            count: 1,
          });
        }
      }

      if let Some(os_version) = profile.os_version {
        if let Some(os_versions) = stats.os_versions
          .iter_mut()
          .find(|f| f.value == os_version)
        {
          os_versions.count += 1;
        } else {
          stats.os_versions.push(Counter {
            value: os_version.into(),
            count: 1,
          });
        }
      }

      if let Some(lang) = profile.lang {
        if let Some(langs) = stats.langs
          .iter_mut()
          .find(|f| f.value == lang)
        {
          langs.count += 1;
        } else {
          stats.langs.push(Counter {
            value: lang.into(),
            count: 1,
          });
        }
      }

      if let Some(cpu64bit) = profile.cpu64bit.map(|f| f.to_string()) {
        if let Some(cpu64bits) = stats.cpu64bits
          .iter_mut()
          .find(|f| f.value == cpu64bit)
        {
          cpu64bits.count += 1;
        } else {
          stats.cpu64bits.push(Counter {
            value: cpu64bit.into(),
            count: 1,
          });
        }
      }

      if let Some(ncpu) = profile.ncpu.map(|f| f.to_string()) {
        if let Some(ncpus) = stats.ncpus
          .iter_mut()
          .find(|f| f.value == ncpu)
        {
          ncpus.count += 1;
        } else {
          stats.ncpus.push(Counter {
            value: ncpu.into(),
            count: 1,
          });
        }
      }

    }

    Metric::upsert(&NewMetric{
      app_id: &app.id,
      date: &end.date().naive_utc(),
      profile_key: "profiles",
      profile_value: "weekly_count",
      count: &numProfiles,
    }, &conn);

    for v in stats.app_versions {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "app_versions",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.cpu64bits {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "cpu64bits",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.ncpus {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "ncpus",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.cpu_freq_mhzs {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "cpu_freq_mhzs",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.cputypes {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "cputypes",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.cpusubtypes {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "cpusubtypes",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.models {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "models",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.ram_mbs {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "ram_mbs",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.os_versions {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "os_versions",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

    for v in stats.langs {
      Metric::upsert(&NewMetric{
        app_id: &app.id,
        date: &end.date().naive_utc(),
        profile_key: "langs",
        profile_value: &v.value,
        count: &v.count,
      }, &conn);
    }

  }
}
