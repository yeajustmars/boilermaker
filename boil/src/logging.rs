use color_eyre::eyre::Result;
use nu_ansi_term::Color;
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;

use util::fmt::{DEBUG_STR, ERROR_STR, INFO_STR, TRACE_STR, WARN_STR};

struct DefaultFormatter;

impl<S, N> FormatEvent<S, N> for DefaultFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        //TODO: filter out some events based on level
        match *event.metadata().level() {
            Level::INFO => write!(&mut writer, "{} ", Color::Blue.paint(INFO_STR))?,
            Level::WARN => write!(&mut writer, "{} ", Color::Yellow.paint(WARN_STR))?,
            Level::ERROR => write!(&mut writer, "{} ", Color::Red.paint(ERROR_STR))?,
            Level::TRACE => write!(&mut writer, "{} ", Color::Purple.paint(TRACE_STR))?,
            Level::DEBUG => write!(&mut writer, "{} ", Color::Purple.paint(DEBUG_STR))?,
        }

        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(&mut writer)?;

        Ok(())
    }
}

#[tracing::instrument]
pub fn init_tracing(debug_level: u8) -> Result<()> {
    //TODO: Add more specific formatting for each debug level (0-4)
    let fmt_layer: Box<dyn tracing_subscriber::Layer<_> + Send + Sync> = match debug_level {
        //TODO: add possible custom formatter to print minimal info (e.g., just the message)
        0 => Box::new(fmt::layer().event_format(DefaultFormatter)),
        //TODO: add custom formatter for debug level 1
        1 => Box::new(
            fmt::layer()
                .event_format(fmt::format().compact())
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_level(true),
        ),
        //TODO: add custom formatter for debug levels 2..=4
        2.. => Box::new(
            fmt::layer()
                .event_format(fmt::format().pretty())
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_level(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true),
        ),
    };

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
