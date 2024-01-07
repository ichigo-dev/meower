//------------------------------------------------------------------------------
//	Constants.
//------------------------------------------------------------------------------

//	Common colors.
const colors =
[
	'red', 'pink', 'purple', 'deep_purple', 'indigo', 'blue', 'light_blue',
	'cyan', 'teal', 'green', 'light_green', 'lime', 'yellow', 'amber',
	'orange', 'deep_orange', 'brown', 'blue_grey', 'grey',
];

const color_types =
[
	'lighten_5', 'lighten_4', 'lighten_3', 'lighten_2', 'lighten_1', '',
	'darken_1', 'darken_2', 'darken_3', 'darken_4',
	'spacer',
	'accent_1', 'accent_2', 'accent_3', 'accent_4',
];

//	Theme colors.
const theme_colors =
[
	'primary', 'secondary', 'error', 'warning', 'info', 'success',
];

const theme_color_types = ['light', '', 'dark'];


//------------------------------------------------------------------------------
//	Shows TOC.
//------------------------------------------------------------------------------
const show_toc = () =>
{
	const elm_headings = document.querySelectorAll('h2, h3');

	const elm_list = document.createElement('ul');
	elm_list.classList.add('ui_list');
	elm_list.classList.add('primary');
	for( elm_heading of elm_headings )
	{
		const elm_list_item_inner = document.createElement('a');
		elm_list_item_inner.setAttribute('href', '#' + elm_heading.id);
		elm_list_item_inner.innerText = elm_heading.innerText;
		elm_list_item_inner.style.display = 'block';
		elm_list_item_inner.style.height = '100%';

		const elm_list_item = document.createElement('li');
		elm_list_item.appendChild(elm_list_item_inner);
		elm_list_item.classList.add('clickable');

		if( elm_heading.tagName.toLowerCase() === 'h3' )
		{
			elm_list_item.classList.add('margin_left_lg');
		}

		elm_list.appendChild(elm_list_item);
	}

	const elm_toc = document.getElementById('toc');
	elm_toc.style.overflow = 'auto';
	elm_toc.appendChild(elm_list);
};


//------------------------------------------------------------------------------
//	Apply the theme.
//------------------------------------------------------------------------------
const apply_theme = ( theme_ ) =>
{
	const theme_wrapper = document.getElementById('theme_wrapper');
	theme_wrapper.className = theme_;
};


//------------------------------------------------------------------------------
//	Gets the text color based on the background color.
//------------------------------------------------------------------------------
const on_text_color = ( color_ ) =>
{
	const background_color = window.getComputedStyle(document.documentElement)
		.getPropertyValue('--color-' + color_.replaceAll('_', '-'));
	const brightness =
	(
		parseInt(background_color.slice(1, 3), 16) * 299 +
		parseInt(background_color.slice(3, 5), 16) * 587 +
		parseInt(background_color.slice(5, 7), 16) * 114
	) / 1000;
	return brightness > 125 ? 'text_black' : 'text_white';
};


//------------------------------------------------------------------------------
//	Creates a color band spacer element.
//------------------------------------------------------------------------------
const color_band_spacer_classes = ['ui_box', 'height_3xs'];
const create_color_band_spacer = () =>
{
	const elm = document.createElement('div');
	for( color_band_spacer_class of color_band_spacer_classes )
	{
		elm.classList.add(color_band_spacer_class);
	}
	return elm;
};


//------------------------------------------------------------------------------
//	Creates a color band element.
//------------------------------------------------------------------------------
const color_band_classes = ['ui_box', 'height_xl', 'width_8xl', 'center'];
const create_color_band = ( color_, type_, fix_text_color_ = false ) =>
{
	const elm = document.createElement('div');
	for( color_band_class of color_band_classes )
	{
		elm.classList.add(color_band_class);
	}

	let color = color_ + (type_.length > 0 ? '_' : '') + type_;
	elm.classList.add(color);
	elm.innerText = color.replaceAll('_', ' ');
	if( fix_text_color_ ) { elm.classList.add(on_text_color(color)); }
	return elm;
};


//------------------------------------------------------------------------------
//	Shows the colors.
//------------------------------------------------------------------------------
const show_colors = () =>
{
	const elm = document.getElementById('colors');
	for( color of colors )
	{
		const elm_palette = document.createElement('div');
		elm_palette.appendChild(create_color_band(color, '', true));
		elm_palette.appendChild(create_color_band_spacer());
		for( color_type of color_types )
		{
			if( color_type == 'spacer' )
			{
				elm_palette.appendChild(create_color_band_spacer());
				continue;
			}

			const elm_color_band = create_color_band(color, color_type, true);
			elm_palette.appendChild(elm_color_band);
		}
		elm.appendChild(elm_palette);
	}
};


//------------------------------------------------------------------------------
//	Shows the theme colors.
//------------------------------------------------------------------------------
const show_theme_colors = () =>
{
	const elm = document.getElementById('theme_colors');
	for( color of theme_colors )
	{
		const elm_palette = document.createElement('div');
		elm_palette.appendChild(create_color_band(color, ''));
		elm_palette.appendChild(create_color_band_spacer());
		for( color_type of theme_color_types )
		{
			elm_palette.appendChild(create_color_band(color, color_type));
		}
		elm.appendChild(elm_palette);
	}
};


//------------------------------------------------------------------------------
//	Initialize.
//------------------------------------------------------------------------------
const init = () =>
{
	show_toc();
	show_colors();
	show_theme_colors();

	//	Apply the theme.
	const elm_theme_select = document.getElementById('theme_select');
	elm_theme_select.addEventListener('change', ( event_ ) =>
	{
		const theme = event_.target.value;
		apply_theme(theme);
	});
	elm_theme_select.dispatchEvent(new Event('change'));

	//	Dialog
	const elm_buttons = document.querySelectorAll('.button_open_dialog');
	elm_buttons.forEach(function(elm_)
	{
		elm_.addEventListener('click', function( event_ )
		{
			const dialog = event_.target.nextElementSibling;
			if( dialog ) { dialog.classList.add('open'); }
		});
	});

	const elm_dialogs = document.querySelectorAll('.ui_dialog');
	elm_dialogs.forEach(function( elm_ )
	{
		elm_.addEventListener('click', function( event_ )
		{
			if( event_.target.closest('.button_close_dialog') )
			{
				elm_.classList.remove('open');
			}

			if( event_.target.closest('.dialog') ) return;
			elm_.classList.remove('open');
		});
	});
};

init();
