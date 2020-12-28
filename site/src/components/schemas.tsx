import React from "react";
import schemaIndex from "../../static/schema_index.json";
import "../__generated__/gatsby-types";

const urlRegex = /(\b(https?|ftp|file):\/\/[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])/gi;

function getUrls(text: string): string[] {
  return text.match(urlRegex);
}

import { Table, List } from "antd";
import { graphql, useStaticQuery } from "gatsby";
import { Breakpoint } from "antd/lib/_util/responsiveObserve";

const columns = [
  {
    title: "Title",
    dataIndex: "title",
    key: "title",
    sorter: (a: any, b: any) => (a.title ?? "").localeCompare(b.title ?? ""),
  },
  {
    title: "Description",
    dataIndex: "description",
    key: "description",
    responsive: ["md"] as Breakpoint[],
    sorter: (a: any, b: any) =>
      (a.description ?? "").localeCompare(b.description ?? ""),
  },
  {
    title: "Last Updated",
    key: "updated",
    dataIndex: "updated",
    responsive: ["sm"] as Breakpoint[],
    render: (updated: string) => (
      <span>{new Date(updated).toLocaleDateString()}</span>
    ),
    sorter: (a: any, b: any) =>
      (new Date(a.updated) ?? new Date()).getTime() -
      (new Date(b.updated) ?? new Date()).getTime(),
  },
  {
    title: "",
    key: "action",
    dataIndex: "url",
    render: (url: string) => <a href={url}>View</a>,
  },
];

export const Schemas: React.FunctionComponent = () => {
  const schemaQuery = useStaticQuery<GatsbyTypes.SchemasQuery>(graphql`
    query Schemas {
      allSchemasJson {
        nodes {
          x_taplo_info {
            authors
            patterns
          }
          parent {
            ... on File {
              name
            }
          }
          description
          title
        }
      }
    }
  `);

  const data = schemaIndex.schemas.map(s => {
    let fileName = s.url.slice(s.url.lastIndexOf("/") + 1);

    const extraInfo = schemaQuery.allSchemasJson.nodes.find(
      s => `${s.parent.name}.json` === fileName
    )!;

    return {
      key: fileName,
      title: s.title ?? "",
      description: s.description ?? "",
      url: s.url,
      updated: s.updated,
      authors: extraInfo.x_taplo_info?.authors ?? [],
      patterns: extraInfo.x_taplo_info?.patterns ?? [],
    };
  });

  return (
    <div>
      {
        <Table
          expandable={{
            expandedRowRender: record => (
              <div>
                <List
                  size="small"
                  header={<div>Authors</div>}
                  itemLayout="horizontal"
                  dataSource={record.authors as string[]}
                  renderItem={author => {
                    const url = getUrls(author)[0];

                    return (
                      <List.Item>
                        {url ? <a href={url}>{author}</a> : <span>author</span>}
                      </List.Item>
                    );
                  }}
                />
                {record.patterns?.length > 0 ? (
                  <List
                    size="small"
                    header={<div>Patterns</div>}
                    itemLayout="horizontal"
                    dataSource={record.patterns as string[]}
                    renderItem={pattern => (
                      <List.Item>
                        {
                          <a
                            href={`https://regexr.com/?expression=/${pattern}/g`}
                          >
                            {pattern}
                          </a>
                        }
                      </List.Item>
                    )}
                  />
                ) : undefined}
              </div>
            ),
            rowExpandable: () => true,
          }}
          pagination={false}
          columns={columns}
          dataSource={data}
        />
      }
    </div>
  );
};
