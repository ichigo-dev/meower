//------------------------------------------------------------------------------
//	Gulp tasks
//------------------------------------------------------------------------------

const gulp = require('gulp');
const browser_sync = require('browser-sync');
const sass = require('gulp-dart-sass');


//------------------------------------------------------------------------------
//	BrowserSync
//------------------------------------------------------------------------------
const browser_sync_start = () =>
{
	browser_sync.init(
	{
		server: './',
		port: 8030,
	});
};


//------------------------------------------------------------------------------
//	BrowserSync Reload
//------------------------------------------------------------------------------
const browser_sync_reload = ( done ) =>
{
	browser_sync.reload();
	done();
};


//------------------------------------------------------------------------------
//	Sass
//------------------------------------------------------------------------------
const compile_sass = () =>
{
	return gulp.src('assets/scss/style.scss')
		.pipe(sass(
		{
			outputStyle: 'compressed',
		}))
		.pipe(gulp.dest('assets/css'))
		.pipe(browser_sync.stream());
};


//------------------------------------------------------------------------------
//	Watch
//------------------------------------------------------------------------------
const watch = () =>
{
	gulp.watch('*.html', gulp.series(browser_sync_reload));
	gulp.watch('assets/css/*.css', gulp.series(browser_sync_reload));
	gulp.watch('assets/scss/**/*.scss', gulp.series(compile_sass));
};


//------------------------------------------------------------------------------
//	Exports
//------------------------------------------------------------------------------
exports.default = gulp.series
(
	gulp.parallel(browser_sync_start, watch)
);
