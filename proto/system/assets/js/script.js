let ui_popups = document.getElementsByClassName('ui_popup');
for( let i = 0; i < ui_popups.length; i++ )
{
    let ui_popup = ui_popups[i];
    let button_close = ui_popup.getElementsByClassName('button_close')[0];
    button_close.addEventListener('click', function()
    {
        ui_popup.classList.remove('show');
    });
}
