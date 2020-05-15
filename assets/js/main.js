let tabs = document.getElementsByClassName('quick-tour-tabs')[0];
let content = document.getElementsByClassName('quick-tour-content')[0];
if (tabs && content) {
    for (let item of tabs.getElementsByClassName('item')) {
        item.addEventListener('click', (event) => {
            event.preventDefault();
            if (!event.target.classList.contains('active')) {
                for (let x of tabs.getElementsByClassName('item')) {
                    x.classList.remove('active');
                }
                for (let x of content.getElementsByClassName('quick-tour-block')) {
                    x.classList.remove('active');
                }
                event.target.classList.add('active');
                document.getElementById(event.target.hash.slice(1)).classList.add('active');
            }
        });
    }
}
