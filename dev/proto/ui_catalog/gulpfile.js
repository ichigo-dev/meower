//------------------------------------------------------------------------------
//	Gulp tasks
//------------------------------------------------------------------------------

const gulp = require("gulp");
const sass = require("gulp-dart-sass");
const browser_sync = require("browser-sync");


//------------------------------------------------------------------------------
//	Sass compile task
//------------------------------------------------------------------------------
const compile_sass = () =>
{
	return gulp.src("assets/scss/style.scss")
		.pipe(sass(
		{
			outputStyle: "compressed",
			errorCss: true,
		}))
		.pipe(gulp.dest("assets/css/"))
		.pipe(browser_sync.stream());
}


//------------------------------------------------------------------------------
//	HTML task
//------------------------------------------------------------------------------
const html = () =>
{
	return gulp.src("index.html")
		.pipe(browser_sync.stream());
}


//------------------------------------------------------------------------------
//	Browser Sync
//------------------------------------------------------------------------------
const browser_sync_server = () =>
{
	browser_sync.init({ server: "./", port: 8030 });
}

const browser_sync_reload = ( done ) =>
{
	browser_sync.reload();
	done();
}


//------------------------------------------------------------------------------
//	Watch files
//------------------------------------------------------------------------------
const watch_files = () =>
{
	gulp.watch("assets/scss/**/*.scss", gulp.series(compile_sass));
	gulp.watch("index.html", gulp.series(html, browser_sync_reload));
}


//------------------------------------------------------------------------------
//	Exports
//------------------------------------------------------------------------------
exports.default = gulp.series
(
	gulp.parallel(html, compile_sass),
	gulp.parallel(watch_files, browser_sync_server),
);
