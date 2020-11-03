export const resolveNavLinkItem = (navbarLink, beforeprefix = "") => {
    const prefix = beforeprefix + (navbarLink.prefix || "");
    const navbarItem = {
        ...navbarLink,
    };
    if (prefix) {
        if (navbarItem.link !== undefined)
            navbarItem.link = prefix + navbarItem.link;
        delete navbarItem.prefix;
    }
    if (navbarItem.items && navbarItem.items.length)
        Object.assign(navbarItem, {
            type: "links",
            items: navbarItem.items.map((item) => resolveNavLinkItem(item, prefix)),
        });
    else
        navbarItem.type = "link";
    return navbarItem;
};
