use super::I18n;

use handlebars::{
    Context,
    Handlebars,
    Helper,
    HelperDef,
    HelperResult,
    Output,
    RenderContext,
    RenderError,
};

pub fn translate(i18n: std::sync::Arc<I18n>) -> Box<dyn HelperDef> {
    Box::new(move |
        helper: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        output: &mut dyn Output,
    | -> HelperResult {
        let locale = helper.param(0)
            .ok_or(RenderError::new("expected locale param"))?
            .value().as_str()
            .ok_or(RenderError::new("expected locale param"))?;

        let key = helper.param(1)
            .ok_or(RenderError::new("expected key param"))?
            .value().as_str()
            .ok_or(RenderError::new("expected key param"))?;

        let i18n = i18n.clone();

        let l10n = i18n.l10n(locale).ok()
            .ok_or(RenderError::new("unknown locale"))?;

        let translated = l10n.translate(key).ok()
            .ok_or(RenderError::new("translation error"))?;

        output.write(&translated)?;

        Ok(())
    })
}
