use hyper::header;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, Context, MiddlewareNext, MiddlewareResult};

// following defaults from github.com/helmetjs/helmet
#[middleware]
pub async fn helmet(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.remove(header::SERVER.as_ref());
    context.set(
        header::CONTENT_SECURITY_POLICY.as_ref(),
        "default-src 'self';base-uri 'self';font-src 'self' https: data:;form-action 'self';frame-ancestors \
         'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' \
         https: 'unsafe-inline';upgrade-insecure-requests",
    );
    context.set("Cross-Origin-Embedder-Policy", "require-corp");
    context.set("Cross-Origin-Opener-Policy", "same-origin");
    context.set("Cross-Origin-Resource-Policy", "same-origin");
    context.set("Origin-Agent-Cluster", "?1");
    context.set(header::REFERRER_POLICY.as_ref(), "no-referrer");
    context.set(header::STRICT_TRANSPORT_SECURITY.as_ref(), "max-age=15552000; includeSubDomains");
    context.set(header::X_CONTENT_TYPE_OPTIONS.as_ref(), "nosniff");
    context.set(header::X_DNS_PREFETCH_CONTROL.as_ref(), "off");
    context.set("X-Download-Options", "noopen");
    context.set(header::X_FRAME_OPTIONS.as_ref(), "SAMEORIGIN");
    context.set("X-Permitted-Cross-Domain-Policies", "none");
    context.set(header::X_XSS_PROTECTION.as_ref(), "0");

    context = next(context).await?;

    Ok(context)
}

#[middleware]
pub async fn cors(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let origin_env = std::env::var("WHITELIST").unwrap_or_else(|_| "*".to_string());

    let origin = if origin_env.contains(',') {
        let header = context
            .hyper_request
            .as_ref()
            .unwrap()
            .request
            .headers()
            .get("Origin")
            .map(|origin| origin.to_str().unwrap().to_string())
            .unwrap_or_else(|| "*".to_string());

        origin_env.split(',').find(|v| v == &header).unwrap_or("")
    } else {
        &origin_env
    };

    context.set("Access-Control-Allow-Origin", origin);
    context.set("Access-Control-Allow-Headers", "*");
    context.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");

    context = next(context).await?;

    Ok(context)
}

// Added in top of helmet defaults
// and OWASP recommendations here https://owasp.org/www-project-secure-headers/#configuration-proposal
#[middleware]
pub async fn recommended_headers_https(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.set("Clear-Site-Data", "*");
    context.set(header::STRICT_TRANSPORT_SECURITY.as_ref(), "max-age=31536000; includeSubDomains");
    context.set(header::X_XSS_PROTECTION.as_ref(), "deny");
    context.set(header::CACHE_CONTROL.as_ref(), "no-store, max-age=0");
    context.set(header::PRAGMA.as_ref(), "no-cache");

    context = next(context).await?;

    Ok(context)
}
