import { AutoComplete, Divider, Input, Layout, Menu } from "antd";
import { SelectProps } from "antd/lib/select";
import { graphql, useStaticQuery } from "gatsby";
import React, { useState } from "react";
import taploIcon from "../assets/taplo-icon.svg";
import CodeIcon from "mdi-react/XmlIcon";
import "../__generated__/gatsby-types";

import SearchIcon from "mdi-react/SearchIcon";

interface NavLink {
  title: string;
  link: string;
  subMenus: SubMenu[];
  icon?: React.ReactNode;
}

const renderIcon = (node: React.ReactNode) => {
  return <span style={{ marginTop: "1rem" }}>{node}</span>;
};

const { Header } = Layout;
const { SubMenu } = Menu;

export interface AppHeaderProps {
  hideLogo?: boolean;
  title?: string;
  path: string;
}

export const AppHeader: React.FunctionComponent<AppHeaderProps> = ({
  path,
  title,
  hideLogo,
}) => {
  const allNavPages = useStaticQuery<GatsbyTypes.NavPagesQuery>(graphql`
    query NavPages {
      allMdx {
        nodes {
          tableOfContents
          slug
          frontmatter {
            nav
            title
          }
        }
      }
    }
  `);

  const staticNavLinks: NavLink[] = [
    {
      title: "Editor",
      link: "editor",
      subMenus: [],
      icon: renderIcon(<CodeIcon size={14} />),
    },
  ];

  const navLinks: NavLink[] = [
    ...staticNavLinks,
    ...allNavPages.allMdx.nodes
      .filter(n => n.frontmatter.nav)
      .map(n => ({
        title: n.frontmatter.nav,
        link: n.slug,
        subMenus:
          (n.tableOfContents as any).items.map(toc => ({
            hash: toc.url,
            title: toc.title,
          })) ?? [],
      })),
  ];

  const currentPage = path.slice(1);

  const [searchOptions, setSearchOptions] = useState<
    SelectProps<object>["options"]
  >([]);

  const renderSearchCategory = (title: string, slug: string) => {
    return (
      <a key={title} href={"/" + slug}>
        {title}
      </a>
    );
  };

  const renderSearchItem = (title: string, link: string, parent: string) => {
    return {
      value: link,
      label: (
        <div
          key={parent + "_" + title}
          style={{
            display: "flex",
            justifyContent: "space-between",
          }}
        >
          {title}
        </div>
      ),
    };
  };

  const searchResult = (value: string) => {
    const lc = value.toLowerCase();

    const opts = [];

    for (const nav of navLinks) {
      const subs = [];

      for (const sub of nav.subMenus) {
        if (sub.title.toLowerCase().indexOf(lc) !== -1) {
          subs.push(
            renderSearchItem(sub.title, nav.link + sub.hash, nav.title)
          );
        }
      }

      if (subs.length > 0 || nav.title.toLowerCase().indexOf(lc) !== -1) {
        opts.push({
          label: renderSearchCategory(nav.title, nav.link),
          options: subs,
        });
      }
    }

    return opts;
  };

  const [searchText, setSearchText] = useState("");

  const handleSearch = (value: string) => {
    setSearchOptions(value?.length ?? 0 > 0 ? searchResult(value) : []);
  };

  const onSearchSelect = (slug: string) => {
    window.location.href = `/${slug}`;
  };

  const createMenu = (nav: NavLink) => {
    if (currentPage === nav.link) {
      return (
        <Menu.Item icon={nav.icon} key={nav.link}>
          {nav.title}
        </Menu.Item>
      );
    }

    if (nav.subMenus.length === 0) {
      return (
        <Menu.Item icon={nav.icon} key={nav.link}>
          <a href={"/" + nav.link}>{nav.title}</a>
        </Menu.Item>
      );
    }

    return (
      <SubMenu
        key={nav.link}
        title={nav.title}
        icon={nav.icon}
        onTitleClick={() => {
          console.log("/" + nav.link);
          window.location.href = "/" + nav.link;
        }}
      >
        {nav.subMenus.map(sub => (
          <Menu.Item key={"/" + nav.link + sub.hash}>
            <a href={"/" + nav.link + sub.hash}>{sub.title}</a>
          </Menu.Item>
        ))}
      </SubMenu>
    );
  };

  const createLogo = () => {
    if (hideLogo) {
      return undefined;
    } else {
      return (
        <>
          <a
            href="/"
            style={{
              display: "flex",
              height: "100%",
              marginLeft: "1rem",
              alignItems: "center",
            }}
          >
            <img src={taploIcon} style={{ height: "80%" }}></img>
            <h1
              style={{
                fontSize: "1.8rem",
                display: "block",
                marginTop: "unset",
                marginBottom: "unset",
                marginLeft: "0.5rem",
              }}
            >
              Taplo
            </h1>
          </a>
          <Divider
            type="vertical"
            style={{ margin: "0px 2rem", height: "80%" }}
          ></Divider>
          <h2
            style={{
              fontSize: "1.3rem",
              marginTop: "unset",
              flexShrink: 0,
              marginBottom: "unset",
              opacity: 0.6,
            }}
          >
            {title}
          </h2>
        </>
      );
    }
  };

  return (
    <Header
      style={{
        padding: 0,
        display: "flex",
        alignItems: "center",
        position: "fixed",
        background: "white",
        width: "100vw",
        boxShadow: "rgba(0, 0, 0, 0.05) 0px 0px 5px 2px",
        zIndex: 1,
      }}
    >
      {createLogo()}
      <div
        style={{
          width: "100%",
          flexShrink: 1,
          display: "flex",
          alignItems: "center",
          justifyContent: "end",
          marginRight: "1rem",
        }}
      >
        <SearchIcon color="#ddddde" />
        <AutoComplete
          dropdownClassName="certain-category-search-dropdown"
          dropdownMatchSelectWidth={500}
          style={{ width: 250 }}
          onSearch={handleSearch}
          onSelect={onSearchSelect}
          autoClearSearchValue={true}
          value={searchText}
          onChange={e => setSearchText(e)}
          options={searchOptions}
        >
          <Input
            size="large"
            bordered={false}
            allowClear={true}
            placeholder="Search Site"
          />
        </AutoComplete>
      </div>
      <Divider type="vertical" style={{ height: "80%" }}></Divider>
      <Menu
        style={{ flexShrink: 0 }}
        selectedKeys={[currentPage]}
        theme="light"
        mode="horizontal"
      >
        {navLinks.map(createMenu)}
      </Menu>
    </Header>
  );
};

interface SubMenu {
  hash: string;
  title: string;
}
