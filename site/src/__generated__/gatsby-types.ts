/* eslint-disable */

declare namespace GatsbyTypes {
type Maybe<T> = T | undefined;
type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  Date: string;
  JSON: never;
};











type BlurredOptions = {
  /** Width of the generated low-res preview. Default is 20px */
  readonly width: Maybe<Scalars['Int']>;
  /**
   * Force the output format for the low-res preview. Default is to use the same
   * format as the input. You should rarely need to change this
   */
  readonly toFormat: Maybe<ImageFormat>;
};

type BooleanQueryOperatorInput = {
  readonly eq: Maybe<Scalars['Boolean']>;
  readonly ne: Maybe<Scalars['Boolean']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};


type DateQueryOperatorInput = {
  readonly eq: Maybe<Scalars['Date']>;
  readonly ne: Maybe<Scalars['Date']>;
  readonly gt: Maybe<Scalars['Date']>;
  readonly gte: Maybe<Scalars['Date']>;
  readonly lt: Maybe<Scalars['Date']>;
  readonly lte: Maybe<Scalars['Date']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['Date']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['Date']>>>;
};

type Directory = Node & {
  readonly sourceInstanceName: Scalars['String'];
  readonly absolutePath: Scalars['String'];
  readonly relativePath: Scalars['String'];
  readonly extension: Scalars['String'];
  readonly size: Scalars['Int'];
  readonly prettySize: Scalars['String'];
  readonly modifiedTime: Scalars['Date'];
  readonly accessTime: Scalars['Date'];
  readonly changeTime: Scalars['Date'];
  readonly birthTime: Scalars['Date'];
  readonly root: Scalars['String'];
  readonly dir: Scalars['String'];
  readonly base: Scalars['String'];
  readonly ext: Scalars['String'];
  readonly name: Scalars['String'];
  readonly relativeDirectory: Scalars['String'];
  readonly dev: Scalars['Int'];
  readonly mode: Scalars['Int'];
  readonly nlink: Scalars['Int'];
  readonly uid: Scalars['Int'];
  readonly gid: Scalars['Int'];
  readonly rdev: Scalars['Int'];
  readonly ino: Scalars['Float'];
  readonly atimeMs: Scalars['Float'];
  readonly mtimeMs: Scalars['Float'];
  readonly ctimeMs: Scalars['Float'];
  readonly atime: Scalars['Date'];
  readonly mtime: Scalars['Date'];
  readonly ctime: Scalars['Date'];
  /** @deprecated Use `birthTime` instead */
  readonly birthtime: Maybe<Scalars['Date']>;
  /** @deprecated Use `birthTime` instead */
  readonly birthtimeMs: Maybe<Scalars['Float']>;
  readonly blksize: Maybe<Scalars['Int']>;
  readonly blocks: Maybe<Scalars['Int']>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
};


type Directory_modifiedTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_accessTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_changeTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_birthTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_atimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_mtimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type Directory_ctimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type DirectoryConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<DirectoryEdge>;
  readonly nodes: ReadonlyArray<Directory>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<DirectoryGroupConnection>;
};


type DirectoryConnection_distinctArgs = {
  field: DirectoryFieldsEnum;
};


type DirectoryConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: DirectoryFieldsEnum;
};

type DirectoryEdge = {
  readonly next: Maybe<Directory>;
  readonly node: Directory;
  readonly previous: Maybe<Directory>;
};

enum DirectoryFieldsEnum {
  sourceInstanceName = 'sourceInstanceName',
  absolutePath = 'absolutePath',
  relativePath = 'relativePath',
  extension = 'extension',
  size = 'size',
  prettySize = 'prettySize',
  modifiedTime = 'modifiedTime',
  accessTime = 'accessTime',
  changeTime = 'changeTime',
  birthTime = 'birthTime',
  root = 'root',
  dir = 'dir',
  base = 'base',
  ext = 'ext',
  name = 'name',
  relativeDirectory = 'relativeDirectory',
  dev = 'dev',
  mode = 'mode',
  nlink = 'nlink',
  uid = 'uid',
  gid = 'gid',
  rdev = 'rdev',
  ino = 'ino',
  atimeMs = 'atimeMs',
  mtimeMs = 'mtimeMs',
  ctimeMs = 'ctimeMs',
  atime = 'atime',
  mtime = 'mtime',
  ctime = 'ctime',
  birthtime = 'birthtime',
  birthtimeMs = 'birthtimeMs',
  blksize = 'blksize',
  blocks = 'blocks',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type'
}

type DirectoryFilterInput = {
  readonly sourceInstanceName: Maybe<StringQueryOperatorInput>;
  readonly absolutePath: Maybe<StringQueryOperatorInput>;
  readonly relativePath: Maybe<StringQueryOperatorInput>;
  readonly extension: Maybe<StringQueryOperatorInput>;
  readonly size: Maybe<IntQueryOperatorInput>;
  readonly prettySize: Maybe<StringQueryOperatorInput>;
  readonly modifiedTime: Maybe<DateQueryOperatorInput>;
  readonly accessTime: Maybe<DateQueryOperatorInput>;
  readonly changeTime: Maybe<DateQueryOperatorInput>;
  readonly birthTime: Maybe<DateQueryOperatorInput>;
  readonly root: Maybe<StringQueryOperatorInput>;
  readonly dir: Maybe<StringQueryOperatorInput>;
  readonly base: Maybe<StringQueryOperatorInput>;
  readonly ext: Maybe<StringQueryOperatorInput>;
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly relativeDirectory: Maybe<StringQueryOperatorInput>;
  readonly dev: Maybe<IntQueryOperatorInput>;
  readonly mode: Maybe<IntQueryOperatorInput>;
  readonly nlink: Maybe<IntQueryOperatorInput>;
  readonly uid: Maybe<IntQueryOperatorInput>;
  readonly gid: Maybe<IntQueryOperatorInput>;
  readonly rdev: Maybe<IntQueryOperatorInput>;
  readonly ino: Maybe<FloatQueryOperatorInput>;
  readonly atimeMs: Maybe<FloatQueryOperatorInput>;
  readonly mtimeMs: Maybe<FloatQueryOperatorInput>;
  readonly ctimeMs: Maybe<FloatQueryOperatorInput>;
  readonly atime: Maybe<DateQueryOperatorInput>;
  readonly mtime: Maybe<DateQueryOperatorInput>;
  readonly ctime: Maybe<DateQueryOperatorInput>;
  readonly birthtime: Maybe<DateQueryOperatorInput>;
  readonly birthtimeMs: Maybe<FloatQueryOperatorInput>;
  readonly blksize: Maybe<IntQueryOperatorInput>;
  readonly blocks: Maybe<IntQueryOperatorInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
};

type DirectoryGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<DirectoryEdge>;
  readonly nodes: ReadonlyArray<Directory>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type DirectorySortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<DirectoryFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type DuotoneGradient = {
  readonly highlight: Scalars['String'];
  readonly shadow: Scalars['String'];
  readonly opacity: Maybe<Scalars['Int']>;
};

type File = Node & {
  readonly sourceInstanceName: Scalars['String'];
  readonly absolutePath: Scalars['String'];
  readonly relativePath: Scalars['String'];
  readonly extension: Scalars['String'];
  readonly size: Scalars['Int'];
  readonly prettySize: Scalars['String'];
  readonly modifiedTime: Scalars['Date'];
  readonly accessTime: Scalars['Date'];
  readonly changeTime: Scalars['Date'];
  readonly birthTime: Scalars['Date'];
  readonly root: Scalars['String'];
  readonly dir: Scalars['String'];
  readonly base: Scalars['String'];
  readonly ext: Scalars['String'];
  readonly name: Scalars['String'];
  readonly relativeDirectory: Scalars['String'];
  readonly dev: Scalars['Int'];
  readonly mode: Scalars['Int'];
  readonly nlink: Scalars['Int'];
  readonly uid: Scalars['Int'];
  readonly gid: Scalars['Int'];
  readonly rdev: Scalars['Int'];
  readonly ino: Scalars['Float'];
  readonly atimeMs: Scalars['Float'];
  readonly mtimeMs: Scalars['Float'];
  readonly ctimeMs: Scalars['Float'];
  readonly atime: Scalars['Date'];
  readonly mtime: Scalars['Date'];
  readonly ctime: Scalars['Date'];
  /** @deprecated Use `birthTime` instead */
  readonly birthtime: Maybe<Scalars['Date']>;
  /** @deprecated Use `birthTime` instead */
  readonly birthtimeMs: Maybe<Scalars['Float']>;
  readonly blksize: Maybe<Scalars['Int']>;
  readonly blocks: Maybe<Scalars['Int']>;
  /** Copy file to static directory and return public url to it */
  readonly publicURL: Maybe<Scalars['String']>;
  readonly childImageSharp: Maybe<ImageSharp>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
  readonly childMdx: Maybe<Mdx>;
  readonly childSchemasJson: Maybe<SchemasJson>;
};


type File_modifiedTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_accessTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_changeTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_birthTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_atimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_mtimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type File_ctimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type FileConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<FileEdge>;
  readonly nodes: ReadonlyArray<File>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<FileGroupConnection>;
};


type FileConnection_distinctArgs = {
  field: FileFieldsEnum;
};


type FileConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: FileFieldsEnum;
};

type FileEdge = {
  readonly next: Maybe<File>;
  readonly node: File;
  readonly previous: Maybe<File>;
};

enum FileFieldsEnum {
  sourceInstanceName = 'sourceInstanceName',
  absolutePath = 'absolutePath',
  relativePath = 'relativePath',
  extension = 'extension',
  size = 'size',
  prettySize = 'prettySize',
  modifiedTime = 'modifiedTime',
  accessTime = 'accessTime',
  changeTime = 'changeTime',
  birthTime = 'birthTime',
  root = 'root',
  dir = 'dir',
  base = 'base',
  ext = 'ext',
  name = 'name',
  relativeDirectory = 'relativeDirectory',
  dev = 'dev',
  mode = 'mode',
  nlink = 'nlink',
  uid = 'uid',
  gid = 'gid',
  rdev = 'rdev',
  ino = 'ino',
  atimeMs = 'atimeMs',
  mtimeMs = 'mtimeMs',
  ctimeMs = 'ctimeMs',
  atime = 'atime',
  mtime = 'mtime',
  ctime = 'ctime',
  birthtime = 'birthtime',
  birthtimeMs = 'birthtimeMs',
  blksize = 'blksize',
  blocks = 'blocks',
  publicURL = 'publicURL',
  childImageSharp___fixed___base64 = 'childImageSharp.fixed.base64',
  childImageSharp___fixed___tracedSVG = 'childImageSharp.fixed.tracedSVG',
  childImageSharp___fixed___aspectRatio = 'childImageSharp.fixed.aspectRatio',
  childImageSharp___fixed___width = 'childImageSharp.fixed.width',
  childImageSharp___fixed___height = 'childImageSharp.fixed.height',
  childImageSharp___fixed___src = 'childImageSharp.fixed.src',
  childImageSharp___fixed___srcSet = 'childImageSharp.fixed.srcSet',
  childImageSharp___fixed___srcWebp = 'childImageSharp.fixed.srcWebp',
  childImageSharp___fixed___srcSetWebp = 'childImageSharp.fixed.srcSetWebp',
  childImageSharp___fixed___originalName = 'childImageSharp.fixed.originalName',
  childImageSharp___resolutions___base64 = 'childImageSharp.resolutions.base64',
  childImageSharp___resolutions___tracedSVG = 'childImageSharp.resolutions.tracedSVG',
  childImageSharp___resolutions___aspectRatio = 'childImageSharp.resolutions.aspectRatio',
  childImageSharp___resolutions___width = 'childImageSharp.resolutions.width',
  childImageSharp___resolutions___height = 'childImageSharp.resolutions.height',
  childImageSharp___resolutions___src = 'childImageSharp.resolutions.src',
  childImageSharp___resolutions___srcSet = 'childImageSharp.resolutions.srcSet',
  childImageSharp___resolutions___srcWebp = 'childImageSharp.resolutions.srcWebp',
  childImageSharp___resolutions___srcSetWebp = 'childImageSharp.resolutions.srcSetWebp',
  childImageSharp___resolutions___originalName = 'childImageSharp.resolutions.originalName',
  childImageSharp___fluid___base64 = 'childImageSharp.fluid.base64',
  childImageSharp___fluid___tracedSVG = 'childImageSharp.fluid.tracedSVG',
  childImageSharp___fluid___aspectRatio = 'childImageSharp.fluid.aspectRatio',
  childImageSharp___fluid___src = 'childImageSharp.fluid.src',
  childImageSharp___fluid___srcSet = 'childImageSharp.fluid.srcSet',
  childImageSharp___fluid___srcWebp = 'childImageSharp.fluid.srcWebp',
  childImageSharp___fluid___srcSetWebp = 'childImageSharp.fluid.srcSetWebp',
  childImageSharp___fluid___sizes = 'childImageSharp.fluid.sizes',
  childImageSharp___fluid___originalImg = 'childImageSharp.fluid.originalImg',
  childImageSharp___fluid___originalName = 'childImageSharp.fluid.originalName',
  childImageSharp___fluid___presentationWidth = 'childImageSharp.fluid.presentationWidth',
  childImageSharp___fluid___presentationHeight = 'childImageSharp.fluid.presentationHeight',
  childImageSharp___sizes___base64 = 'childImageSharp.sizes.base64',
  childImageSharp___sizes___tracedSVG = 'childImageSharp.sizes.tracedSVG',
  childImageSharp___sizes___aspectRatio = 'childImageSharp.sizes.aspectRatio',
  childImageSharp___sizes___src = 'childImageSharp.sizes.src',
  childImageSharp___sizes___srcSet = 'childImageSharp.sizes.srcSet',
  childImageSharp___sizes___srcWebp = 'childImageSharp.sizes.srcWebp',
  childImageSharp___sizes___srcSetWebp = 'childImageSharp.sizes.srcSetWebp',
  childImageSharp___sizes___sizes = 'childImageSharp.sizes.sizes',
  childImageSharp___sizes___originalImg = 'childImageSharp.sizes.originalImg',
  childImageSharp___sizes___originalName = 'childImageSharp.sizes.originalName',
  childImageSharp___sizes___presentationWidth = 'childImageSharp.sizes.presentationWidth',
  childImageSharp___sizes___presentationHeight = 'childImageSharp.sizes.presentationHeight',
  childImageSharp___gatsbyImageData = 'childImageSharp.gatsbyImageData',
  childImageSharp___original___width = 'childImageSharp.original.width',
  childImageSharp___original___height = 'childImageSharp.original.height',
  childImageSharp___original___src = 'childImageSharp.original.src',
  childImageSharp___resize___src = 'childImageSharp.resize.src',
  childImageSharp___resize___tracedSVG = 'childImageSharp.resize.tracedSVG',
  childImageSharp___resize___width = 'childImageSharp.resize.width',
  childImageSharp___resize___height = 'childImageSharp.resize.height',
  childImageSharp___resize___aspectRatio = 'childImageSharp.resize.aspectRatio',
  childImageSharp___resize___originalName = 'childImageSharp.resize.originalName',
  childImageSharp___id = 'childImageSharp.id',
  childImageSharp___parent___id = 'childImageSharp.parent.id',
  childImageSharp___parent___parent___id = 'childImageSharp.parent.parent.id',
  childImageSharp___parent___parent___children = 'childImageSharp.parent.parent.children',
  childImageSharp___parent___children = 'childImageSharp.parent.children',
  childImageSharp___parent___children___id = 'childImageSharp.parent.children.id',
  childImageSharp___parent___children___children = 'childImageSharp.parent.children.children',
  childImageSharp___parent___internal___content = 'childImageSharp.parent.internal.content',
  childImageSharp___parent___internal___contentDigest = 'childImageSharp.parent.internal.contentDigest',
  childImageSharp___parent___internal___description = 'childImageSharp.parent.internal.description',
  childImageSharp___parent___internal___fieldOwners = 'childImageSharp.parent.internal.fieldOwners',
  childImageSharp___parent___internal___ignoreType = 'childImageSharp.parent.internal.ignoreType',
  childImageSharp___parent___internal___mediaType = 'childImageSharp.parent.internal.mediaType',
  childImageSharp___parent___internal___owner = 'childImageSharp.parent.internal.owner',
  childImageSharp___parent___internal___type = 'childImageSharp.parent.internal.type',
  childImageSharp___children = 'childImageSharp.children',
  childImageSharp___children___id = 'childImageSharp.children.id',
  childImageSharp___children___parent___id = 'childImageSharp.children.parent.id',
  childImageSharp___children___parent___children = 'childImageSharp.children.parent.children',
  childImageSharp___children___children = 'childImageSharp.children.children',
  childImageSharp___children___children___id = 'childImageSharp.children.children.id',
  childImageSharp___children___children___children = 'childImageSharp.children.children.children',
  childImageSharp___children___internal___content = 'childImageSharp.children.internal.content',
  childImageSharp___children___internal___contentDigest = 'childImageSharp.children.internal.contentDigest',
  childImageSharp___children___internal___description = 'childImageSharp.children.internal.description',
  childImageSharp___children___internal___fieldOwners = 'childImageSharp.children.internal.fieldOwners',
  childImageSharp___children___internal___ignoreType = 'childImageSharp.children.internal.ignoreType',
  childImageSharp___children___internal___mediaType = 'childImageSharp.children.internal.mediaType',
  childImageSharp___children___internal___owner = 'childImageSharp.children.internal.owner',
  childImageSharp___children___internal___type = 'childImageSharp.children.internal.type',
  childImageSharp___internal___content = 'childImageSharp.internal.content',
  childImageSharp___internal___contentDigest = 'childImageSharp.internal.contentDigest',
  childImageSharp___internal___description = 'childImageSharp.internal.description',
  childImageSharp___internal___fieldOwners = 'childImageSharp.internal.fieldOwners',
  childImageSharp___internal___ignoreType = 'childImageSharp.internal.ignoreType',
  childImageSharp___internal___mediaType = 'childImageSharp.internal.mediaType',
  childImageSharp___internal___owner = 'childImageSharp.internal.owner',
  childImageSharp___internal___type = 'childImageSharp.internal.type',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type',
  childMdx___rawBody = 'childMdx.rawBody',
  childMdx___fileAbsolutePath = 'childMdx.fileAbsolutePath',
  childMdx___frontmatter___title = 'childMdx.frontmatter.title',
  childMdx___frontmatter___nav = 'childMdx.frontmatter.nav',
  childMdx___frontmatter___navOrder = 'childMdx.frontmatter.navOrder',
  childMdx___slug = 'childMdx.slug',
  childMdx___body = 'childMdx.body',
  childMdx___excerpt = 'childMdx.excerpt',
  childMdx___headings = 'childMdx.headings',
  childMdx___headings___value = 'childMdx.headings.value',
  childMdx___headings___depth = 'childMdx.headings.depth',
  childMdx___html = 'childMdx.html',
  childMdx___mdxAST = 'childMdx.mdxAST',
  childMdx___tableOfContents = 'childMdx.tableOfContents',
  childMdx___timeToRead = 'childMdx.timeToRead',
  childMdx___wordCount___paragraphs = 'childMdx.wordCount.paragraphs',
  childMdx___wordCount___sentences = 'childMdx.wordCount.sentences',
  childMdx___wordCount___words = 'childMdx.wordCount.words',
  childMdx___id = 'childMdx.id',
  childMdx___parent___id = 'childMdx.parent.id',
  childMdx___parent___parent___id = 'childMdx.parent.parent.id',
  childMdx___parent___parent___children = 'childMdx.parent.parent.children',
  childMdx___parent___children = 'childMdx.parent.children',
  childMdx___parent___children___id = 'childMdx.parent.children.id',
  childMdx___parent___children___children = 'childMdx.parent.children.children',
  childMdx___parent___internal___content = 'childMdx.parent.internal.content',
  childMdx___parent___internal___contentDigest = 'childMdx.parent.internal.contentDigest',
  childMdx___parent___internal___description = 'childMdx.parent.internal.description',
  childMdx___parent___internal___fieldOwners = 'childMdx.parent.internal.fieldOwners',
  childMdx___parent___internal___ignoreType = 'childMdx.parent.internal.ignoreType',
  childMdx___parent___internal___mediaType = 'childMdx.parent.internal.mediaType',
  childMdx___parent___internal___owner = 'childMdx.parent.internal.owner',
  childMdx___parent___internal___type = 'childMdx.parent.internal.type',
  childMdx___children = 'childMdx.children',
  childMdx___children___id = 'childMdx.children.id',
  childMdx___children___parent___id = 'childMdx.children.parent.id',
  childMdx___children___parent___children = 'childMdx.children.parent.children',
  childMdx___children___children = 'childMdx.children.children',
  childMdx___children___children___id = 'childMdx.children.children.id',
  childMdx___children___children___children = 'childMdx.children.children.children',
  childMdx___children___internal___content = 'childMdx.children.internal.content',
  childMdx___children___internal___contentDigest = 'childMdx.children.internal.contentDigest',
  childMdx___children___internal___description = 'childMdx.children.internal.description',
  childMdx___children___internal___fieldOwners = 'childMdx.children.internal.fieldOwners',
  childMdx___children___internal___ignoreType = 'childMdx.children.internal.ignoreType',
  childMdx___children___internal___mediaType = 'childMdx.children.internal.mediaType',
  childMdx___children___internal___owner = 'childMdx.children.internal.owner',
  childMdx___children___internal___type = 'childMdx.children.internal.type',
  childMdx___internal___content = 'childMdx.internal.content',
  childMdx___internal___contentDigest = 'childMdx.internal.contentDigest',
  childMdx___internal___description = 'childMdx.internal.description',
  childMdx___internal___fieldOwners = 'childMdx.internal.fieldOwners',
  childMdx___internal___ignoreType = 'childMdx.internal.ignoreType',
  childMdx___internal___mediaType = 'childMdx.internal.mediaType',
  childMdx___internal___owner = 'childMdx.internal.owner',
  childMdx___internal___type = 'childMdx.internal.type',
  childSchemasJson___id = 'childSchemasJson.id',
  childSchemasJson___parent___id = 'childSchemasJson.parent.id',
  childSchemasJson___parent___parent___id = 'childSchemasJson.parent.parent.id',
  childSchemasJson___parent___parent___children = 'childSchemasJson.parent.parent.children',
  childSchemasJson___parent___children = 'childSchemasJson.parent.children',
  childSchemasJson___parent___children___id = 'childSchemasJson.parent.children.id',
  childSchemasJson___parent___children___children = 'childSchemasJson.parent.children.children',
  childSchemasJson___parent___internal___content = 'childSchemasJson.parent.internal.content',
  childSchemasJson___parent___internal___contentDigest = 'childSchemasJson.parent.internal.contentDigest',
  childSchemasJson___parent___internal___description = 'childSchemasJson.parent.internal.description',
  childSchemasJson___parent___internal___fieldOwners = 'childSchemasJson.parent.internal.fieldOwners',
  childSchemasJson___parent___internal___ignoreType = 'childSchemasJson.parent.internal.ignoreType',
  childSchemasJson___parent___internal___mediaType = 'childSchemasJson.parent.internal.mediaType',
  childSchemasJson___parent___internal___owner = 'childSchemasJson.parent.internal.owner',
  childSchemasJson___parent___internal___type = 'childSchemasJson.parent.internal.type',
  childSchemasJson___children = 'childSchemasJson.children',
  childSchemasJson___children___id = 'childSchemasJson.children.id',
  childSchemasJson___children___parent___id = 'childSchemasJson.children.parent.id',
  childSchemasJson___children___parent___children = 'childSchemasJson.children.parent.children',
  childSchemasJson___children___children = 'childSchemasJson.children.children',
  childSchemasJson___children___children___id = 'childSchemasJson.children.children.id',
  childSchemasJson___children___children___children = 'childSchemasJson.children.children.children',
  childSchemasJson___children___internal___content = 'childSchemasJson.children.internal.content',
  childSchemasJson___children___internal___contentDigest = 'childSchemasJson.children.internal.contentDigest',
  childSchemasJson___children___internal___description = 'childSchemasJson.children.internal.description',
  childSchemasJson___children___internal___fieldOwners = 'childSchemasJson.children.internal.fieldOwners',
  childSchemasJson___children___internal___ignoreType = 'childSchemasJson.children.internal.ignoreType',
  childSchemasJson___children___internal___mediaType = 'childSchemasJson.children.internal.mediaType',
  childSchemasJson___children___internal___owner = 'childSchemasJson.children.internal.owner',
  childSchemasJson___children___internal___type = 'childSchemasJson.children.internal.type',
  childSchemasJson___internal___content = 'childSchemasJson.internal.content',
  childSchemasJson___internal___contentDigest = 'childSchemasJson.internal.contentDigest',
  childSchemasJson___internal___description = 'childSchemasJson.internal.description',
  childSchemasJson___internal___fieldOwners = 'childSchemasJson.internal.fieldOwners',
  childSchemasJson___internal___ignoreType = 'childSchemasJson.internal.ignoreType',
  childSchemasJson___internal___mediaType = 'childSchemasJson.internal.mediaType',
  childSchemasJson___internal___owner = 'childSchemasJson.internal.owner',
  childSchemasJson___internal___type = 'childSchemasJson.internal.type',
  childSchemasJson____schema = 'childSchemasJson._schema',
  childSchemasJson___title = 'childSchemasJson.title',
  childSchemasJson___type = 'childSchemasJson.type',
  childSchemasJson___description = 'childSchemasJson.description',
  childSchemasJson___x_taplo_info___authors = 'childSchemasJson.x_taplo_info.authors',
  childSchemasJson___x_taplo_info___patterns = 'childSchemasJson.x_taplo_info.patterns',
  childSchemasJson___properties___tab_spaces___type = 'childSchemasJson.properties.tab_spaces.type',
  childSchemasJson___properties___tab_spaces___description = 'childSchemasJson.properties.tab_spaces.description',
  childSchemasJson___properties___tab_spaces___default = 'childSchemasJson.properties.tab_spaces.default',
  childSchemasJson___properties___fn_args_layout___type = 'childSchemasJson.properties.fn_args_layout.type',
  childSchemasJson___properties___fn_args_layout___description = 'childSchemasJson.properties.fn_args_layout.description',
  childSchemasJson___properties___fn_args_layout___default = 'childSchemasJson.properties.fn_args_layout.default',
  childSchemasJson___properties___fn_args_layout___enum = 'childSchemasJson.properties.fn_args_layout.enum',
  childSchemasJson___properties___merge_derives___type = 'childSchemasJson.properties.merge_derives.type',
  childSchemasJson___properties___merge_derives___description = 'childSchemasJson.properties.merge_derives.description',
  childSchemasJson___properties___merge_derives___default = 'childSchemasJson.properties.merge_derives.default',
  childSchemasJson___properties___merge_derives___enum = 'childSchemasJson.properties.merge_derives.enum',
  childSchemasJson___properties___print_misformatted_file_names___type = 'childSchemasJson.properties.print_misformatted_file_names.type',
  childSchemasJson___properties___print_misformatted_file_names___description = 'childSchemasJson.properties.print_misformatted_file_names.description',
  childSchemasJson___properties___print_misformatted_file_names___default = 'childSchemasJson.properties.print_misformatted_file_names.default',
  childSchemasJson___properties___print_misformatted_file_names___enum = 'childSchemasJson.properties.print_misformatted_file_names.enum',
  childSchemasJson___properties___remove_nested_parens___type = 'childSchemasJson.properties.remove_nested_parens.type',
  childSchemasJson___properties___remove_nested_parens___description = 'childSchemasJson.properties.remove_nested_parens.description',
  childSchemasJson___properties___remove_nested_parens___default = 'childSchemasJson.properties.remove_nested_parens.default',
  childSchemasJson___properties___remove_nested_parens___enum = 'childSchemasJson.properties.remove_nested_parens.enum',
  childSchemasJson___properties___use_small_heuristics___type = 'childSchemasJson.properties.use_small_heuristics.type',
  childSchemasJson___properties___use_small_heuristics___description = 'childSchemasJson.properties.use_small_heuristics.description',
  childSchemasJson___properties___use_small_heuristics___default = 'childSchemasJson.properties.use_small_heuristics.default',
  childSchemasJson___properties___use_small_heuristics___enum = 'childSchemasJson.properties.use_small_heuristics.enum',
  childSchemasJson___properties___use_try_shorthand___type = 'childSchemasJson.properties.use_try_shorthand.type',
  childSchemasJson___properties___use_try_shorthand___description = 'childSchemasJson.properties.use_try_shorthand.description',
  childSchemasJson___properties___use_try_shorthand___default = 'childSchemasJson.properties.use_try_shorthand.default',
  childSchemasJson___properties___use_try_shorthand___enum = 'childSchemasJson.properties.use_try_shorthand.enum',
  childSchemasJson___properties___reorder_modules___type = 'childSchemasJson.properties.reorder_modules.type',
  childSchemasJson___properties___reorder_modules___description = 'childSchemasJson.properties.reorder_modules.description',
  childSchemasJson___properties___reorder_modules___default = 'childSchemasJson.properties.reorder_modules.default',
  childSchemasJson___properties___reorder_modules___enum = 'childSchemasJson.properties.reorder_modules.enum',
  childSchemasJson___properties___hard_tabs___type = 'childSchemasJson.properties.hard_tabs.type',
  childSchemasJson___properties___hard_tabs___description = 'childSchemasJson.properties.hard_tabs.description',
  childSchemasJson___properties___hard_tabs___default = 'childSchemasJson.properties.hard_tabs.default',
  childSchemasJson___properties___hard_tabs___enum = 'childSchemasJson.properties.hard_tabs.enum',
  childSchemasJson___properties___use_field_init_shorthand___type = 'childSchemasJson.properties.use_field_init_shorthand.type',
  childSchemasJson___properties___use_field_init_shorthand___description = 'childSchemasJson.properties.use_field_init_shorthand.description',
  childSchemasJson___properties___use_field_init_shorthand___default = 'childSchemasJson.properties.use_field_init_shorthand.default',
  childSchemasJson___properties___use_field_init_shorthand___enum = 'childSchemasJson.properties.use_field_init_shorthand.enum',
  childSchemasJson___properties___max_width___type = 'childSchemasJson.properties.max_width.type',
  childSchemasJson___properties___max_width___description = 'childSchemasJson.properties.max_width.description',
  childSchemasJson___properties___max_width___default = 'childSchemasJson.properties.max_width.default',
  childSchemasJson___properties___reorder_imports___type = 'childSchemasJson.properties.reorder_imports.type',
  childSchemasJson___properties___reorder_imports___description = 'childSchemasJson.properties.reorder_imports.description',
  childSchemasJson___properties___reorder_imports___default = 'childSchemasJson.properties.reorder_imports.default',
  childSchemasJson___properties___reorder_imports___enum = 'childSchemasJson.properties.reorder_imports.enum',
  childSchemasJson___properties___match_arm_leading_pipes___type = 'childSchemasJson.properties.match_arm_leading_pipes.type',
  childSchemasJson___properties___match_arm_leading_pipes___description = 'childSchemasJson.properties.match_arm_leading_pipes.description',
  childSchemasJson___properties___match_arm_leading_pipes___default = 'childSchemasJson.properties.match_arm_leading_pipes.default',
  childSchemasJson___properties___match_arm_leading_pipes___enum = 'childSchemasJson.properties.match_arm_leading_pipes.enum',
  childSchemasJson___properties___force_explicit_abi___type = 'childSchemasJson.properties.force_explicit_abi.type',
  childSchemasJson___properties___force_explicit_abi___description = 'childSchemasJson.properties.force_explicit_abi.description',
  childSchemasJson___properties___force_explicit_abi___default = 'childSchemasJson.properties.force_explicit_abi.default',
  childSchemasJson___properties___force_explicit_abi___enum = 'childSchemasJson.properties.force_explicit_abi.enum',
  childSchemasJson___properties___edition___type = 'childSchemasJson.properties.edition.type',
  childSchemasJson___properties___edition___description = 'childSchemasJson.properties.edition.description',
  childSchemasJson___properties___edition___default = 'childSchemasJson.properties.edition.default',
  childSchemasJson___properties___edition___enum = 'childSchemasJson.properties.edition.enum',
  childSchemasJson___properties___newline_style___type = 'childSchemasJson.properties.newline_style.type',
  childSchemasJson___properties___newline_style___description = 'childSchemasJson.properties.newline_style.description',
  childSchemasJson___properties___newline_style___default = 'childSchemasJson.properties.newline_style.default',
  childSchemasJson___properties___newline_style___enum = 'childSchemasJson.properties.newline_style.enum',
  childSchemasJson___properties___build_system____ref = 'childSchemasJson.properties.build_system._ref',
  childSchemasJson___properties___tool___type = 'childSchemasJson.properties.tool.type',
  childSchemasJson___properties___tool___description = 'childSchemasJson.properties.tool.description',
  childSchemasJson___properties___tool___additionalProperties = 'childSchemasJson.properties.tool.additionalProperties',
  childSchemasJson___properties___badges___description = 'childSchemasJson.properties.badges.description',
  childSchemasJson___properties___badges___type = 'childSchemasJson.properties.badges.type',
  childSchemasJson___properties___bench___description = 'childSchemasJson.properties.bench.description',
  childSchemasJson___properties___bench___type = 'childSchemasJson.properties.bench.type',
  childSchemasJson___properties___bin___description = 'childSchemasJson.properties.bin.description',
  childSchemasJson___properties___bin___type = 'childSchemasJson.properties.bin.type',
  childSchemasJson___properties___build_dependencies___type = 'childSchemasJson.properties.build_dependencies.type',
  childSchemasJson___properties___cargo_features___type = 'childSchemasJson.properties.cargo_features.type',
  childSchemasJson___properties___dependencies___description = 'childSchemasJson.properties.dependencies.description',
  childSchemasJson___properties___dependencies___type = 'childSchemasJson.properties.dependencies.type',
  childSchemasJson___properties___dev_dependencies___type = 'childSchemasJson.properties.dev_dependencies.type',
  childSchemasJson___properties___example___description = 'childSchemasJson.properties.example.description',
  childSchemasJson___properties___example___type = 'childSchemasJson.properties.example.type',
  childSchemasJson___properties___features___description = 'childSchemasJson.properties.features.description',
  childSchemasJson___properties___features___type = 'childSchemasJson.properties.features.type',
  childSchemasJson___properties___lib____ref = 'childSchemasJson.properties.lib._ref',
  childSchemasJson___properties___package____ref = 'childSchemasJson.properties.package._ref',
  childSchemasJson___properties___patch___description = 'childSchemasJson.properties.patch.description',
  childSchemasJson___properties___patch___type = 'childSchemasJson.properties.patch.type',
  childSchemasJson___properties___profile____ref = 'childSchemasJson.properties.profile._ref',
  childSchemasJson___properties___project____ref = 'childSchemasJson.properties.project._ref',
  childSchemasJson___properties___replace___type = 'childSchemasJson.properties.replace.type',
  childSchemasJson___properties___target___type = 'childSchemasJson.properties.target.type',
  childSchemasJson___properties___test___description = 'childSchemasJson.properties.test.description',
  childSchemasJson___properties___test___type = 'childSchemasJson.properties.test.type',
  childSchemasJson___properties___workspace____ref = 'childSchemasJson.properties.workspace._ref',
  childSchemasJson___definitions___poetry_author_pattern___description = 'childSchemasJson.definitions.poetry_author_pattern.description',
  childSchemasJson___definitions___poetry_author_pattern___type = 'childSchemasJson.definitions.poetry_author_pattern.type',
  childSchemasJson___definitions___poetry_author_pattern___pattern = 'childSchemasJson.definitions.poetry_author_pattern.pattern',
  childSchemasJson___definitions___poetry_authors___type = 'childSchemasJson.definitions.poetry_authors.type',
  childSchemasJson___definitions___poetry_authors___description = 'childSchemasJson.definitions.poetry_authors.description',
  childSchemasJson___definitions___poetry_maintainers___type = 'childSchemasJson.definitions.poetry_maintainers.type',
  childSchemasJson___definitions___poetry_maintainers___description = 'childSchemasJson.definitions.poetry_maintainers.description',
  childSchemasJson___definitions___poetry_dependency_any___oneOf = 'childSchemasJson.definitions.poetry_dependency_any.oneOf',
  childSchemasJson___definitions___poetry_pep440_version___type = 'childSchemasJson.definitions.poetry_pep440_version.type',
  childSchemasJson___definitions___poetry_pep440_version___description = 'childSchemasJson.definitions.poetry_pep440_version.description',
  childSchemasJson___definitions___poetry_dependency____ref = 'childSchemasJson.definitions.poetry_dependency._ref',
  childSchemasJson___definitions___poetry_long_dependency___type = 'childSchemasJson.definitions.poetry_long_dependency.type',
  childSchemasJson___definitions___poetry_long_dependency___required = 'childSchemasJson.definitions.poetry_long_dependency.required',
  childSchemasJson___definitions___poetry_long_dependency___additionalProperties = 'childSchemasJson.definitions.poetry_long_dependency.additionalProperties',
  childSchemasJson___definitions___poetry_git_dependency___type = 'childSchemasJson.definitions.poetry_git_dependency.type',
  childSchemasJson___definitions___poetry_git_dependency___required = 'childSchemasJson.definitions.poetry_git_dependency.required',
  childSchemasJson___definitions___poetry_git_dependency___additionalProperties = 'childSchemasJson.definitions.poetry_git_dependency.additionalProperties',
  childSchemasJson___definitions___poetry_file_dependency___type = 'childSchemasJson.definitions.poetry_file_dependency.type',
  childSchemasJson___definitions___poetry_file_dependency___required = 'childSchemasJson.definitions.poetry_file_dependency.required',
  childSchemasJson___definitions___poetry_file_dependency___additionalProperties = 'childSchemasJson.definitions.poetry_file_dependency.additionalProperties',
  childSchemasJson___definitions___poetry_path_dependency___type = 'childSchemasJson.definitions.poetry_path_dependency.type',
  childSchemasJson___definitions___poetry_path_dependency___required = 'childSchemasJson.definitions.poetry_path_dependency.required',
  childSchemasJson___definitions___poetry_path_dependency___additionalProperties = 'childSchemasJson.definitions.poetry_path_dependency.additionalProperties',
  childSchemasJson___definitions___poetry_url_dependency___type = 'childSchemasJson.definitions.poetry_url_dependency.type',
  childSchemasJson___definitions___poetry_url_dependency___required = 'childSchemasJson.definitions.poetry_url_dependency.required',
  childSchemasJson___definitions___poetry_url_dependency___additionalProperties = 'childSchemasJson.definitions.poetry_url_dependency.additionalProperties',
  childSchemasJson___definitions___poetry_multiple_constraints_dependency___type = 'childSchemasJson.definitions.poetry_multiple_constraints_dependency.type',
  childSchemasJson___definitions___poetry_multiple_constraints_dependency___minItems = 'childSchemasJson.definitions.poetry_multiple_constraints_dependency.minItems',
  childSchemasJson___definitions___poetry_scripts___type = 'childSchemasJson.definitions.poetry_scripts.type',
  childSchemasJson___definitions___poetry_script___type = 'childSchemasJson.definitions.poetry_script.type',
  childSchemasJson___definitions___poetry_script___description = 'childSchemasJson.definitions.poetry_script.description',
  childSchemasJson___definitions___poetry_extra_script___type = 'childSchemasJson.definitions.poetry_extra_script.type',
  childSchemasJson___definitions___poetry_extra_script___description = 'childSchemasJson.definitions.poetry_extra_script.description',
  childSchemasJson___definitions___poetry_extra_script___additionalProperties = 'childSchemasJson.definitions.poetry_extra_script.additionalProperties',
  childSchemasJson___definitions___poetry_repository___type = 'childSchemasJson.definitions.poetry_repository.type',
  childSchemasJson___definitions___poetry_repository___additionalProperties = 'childSchemasJson.definitions.poetry_repository.additionalProperties',
  childSchemasJson___definitions___BuildSystem___title = 'childSchemasJson.definitions.BuildSystem.title',
  childSchemasJson___definitions___BuildSystem___type = 'childSchemasJson.definitions.BuildSystem.type',
  childSchemasJson___definitions___BuildSystem___description = 'childSchemasJson.definitions.BuildSystem.description',
  childSchemasJson___definitions___BuildSystem___required = 'childSchemasJson.definitions.BuildSystem.required',
  childSchemasJson___definitions___Build___title = 'childSchemasJson.definitions.Build.title',
  childSchemasJson___definitions___Build___description = 'childSchemasJson.definitions.Build.description',
  childSchemasJson___definitions___Build___anyOf = 'childSchemasJson.definitions.Build.anyOf',
  childSchemasJson___definitions___DebugLevel___title = 'childSchemasJson.definitions.DebugLevel.title',
  childSchemasJson___definitions___DebugLevel___description = 'childSchemasJson.definitions.DebugLevel.description',
  childSchemasJson___definitions___DebugLevel___anyOf = 'childSchemasJson.definitions.DebugLevel.anyOf',
  childSchemasJson___definitions___Dependency___title = 'childSchemasJson.definitions.Dependency.title',
  childSchemasJson___definitions___Dependency___anyOf = 'childSchemasJson.definitions.Dependency.anyOf',
  childSchemasJson___definitions___DetailedDependency___title = 'childSchemasJson.definitions.DetailedDependency.title',
  childSchemasJson___definitions___DetailedDependency___type = 'childSchemasJson.definitions.DetailedDependency.type',
  childSchemasJson___definitions___Edition___title = 'childSchemasJson.definitions.Edition.title',
  childSchemasJson___definitions___Edition___description = 'childSchemasJson.definitions.Edition.description',
  childSchemasJson___definitions___Edition___type = 'childSchemasJson.definitions.Edition.type',
  childSchemasJson___definitions___Edition___enum = 'childSchemasJson.definitions.Edition.enum',
  childSchemasJson___definitions___Lto___title = 'childSchemasJson.definitions.Lto.title',
  childSchemasJson___definitions___Lto___description = 'childSchemasJson.definitions.Lto.description',
  childSchemasJson___definitions___Lto___anyOf = 'childSchemasJson.definitions.Lto.anyOf',
  childSchemasJson___definitions___MetaBuild___title = 'childSchemasJson.definitions.MetaBuild.title',
  childSchemasJson___definitions___MetaBuild___type = 'childSchemasJson.definitions.MetaBuild.type',
  childSchemasJson___definitions___OptLevel___title = 'childSchemasJson.definitions.OptLevel.title',
  childSchemasJson___definitions___OptLevel___description = 'childSchemasJson.definitions.OptLevel.description',
  childSchemasJson___definitions___OptLevel___anyOf = 'childSchemasJson.definitions.OptLevel.anyOf',
  childSchemasJson___definitions___Package___title = 'childSchemasJson.definitions.Package.title',
  childSchemasJson___definitions___Package___description = 'childSchemasJson.definitions.Package.description',
  childSchemasJson___definitions___Package___type = 'childSchemasJson.definitions.Package.type',
  childSchemasJson___definitions___Package___required = 'childSchemasJson.definitions.Package.required',
  childSchemasJson___definitions___Panic___title = 'childSchemasJson.definitions.Panic.title',
  childSchemasJson___definitions___Panic___description = 'childSchemasJson.definitions.Panic.description',
  childSchemasJson___definitions___Panic___type = 'childSchemasJson.definitions.Panic.type',
  childSchemasJson___definitions___Panic___enum = 'childSchemasJson.definitions.Panic.enum',
  childSchemasJson___definitions___Platform___title = 'childSchemasJson.definitions.Platform.title',
  childSchemasJson___definitions___Platform___type = 'childSchemasJson.definitions.Platform.type',
  childSchemasJson___definitions___Profile___title = 'childSchemasJson.definitions.Profile.title',
  childSchemasJson___definitions___Profile___type = 'childSchemasJson.definitions.Profile.type',
  childSchemasJson___definitions___Profiles___title = 'childSchemasJson.definitions.Profiles.title',
  childSchemasJson___definitions___Profiles___description = 'childSchemasJson.definitions.Profiles.description',
  childSchemasJson___definitions___Profiles___type = 'childSchemasJson.definitions.Profiles.type',
  childSchemasJson___definitions___Publish___title = 'childSchemasJson.definitions.Publish.title',
  childSchemasJson___definitions___Publish___description = 'childSchemasJson.definitions.Publish.description',
  childSchemasJson___definitions___Publish___anyOf = 'childSchemasJson.definitions.Publish.anyOf',
  childSchemasJson___definitions___Readme___title = 'childSchemasJson.definitions.Readme.title',
  childSchemasJson___definitions___Readme___description = 'childSchemasJson.definitions.Readme.description',
  childSchemasJson___definitions___Readme___anyOf = 'childSchemasJson.definitions.Readme.anyOf',
  childSchemasJson___definitions___SemVer___title = 'childSchemasJson.definitions.SemVer.title',
  childSchemasJson___definitions___SemVer___description = 'childSchemasJson.definitions.SemVer.description',
  childSchemasJson___definitions___SemVer___default = 'childSchemasJson.definitions.SemVer.default',
  childSchemasJson___definitions___SemVer___type = 'childSchemasJson.definitions.SemVer.type',
  childSchemasJson___definitions___SemVer___pattern = 'childSchemasJson.definitions.SemVer.pattern',
  childSchemasJson___definitions___SemVerRequirement___title = 'childSchemasJson.definitions.SemVerRequirement.title',
  childSchemasJson___definitions___SemVerRequirement___description = 'childSchemasJson.definitions.SemVerRequirement.description',
  childSchemasJson___definitions___SemVerRequirement___default = 'childSchemasJson.definitions.SemVerRequirement.default',
  childSchemasJson___definitions___SemVerRequirement___type = 'childSchemasJson.definitions.SemVerRequirement.type',
  childSchemasJson___definitions___Target___title = 'childSchemasJson.definitions.Target.title',
  childSchemasJson___definitions___Target___type = 'childSchemasJson.definitions.Target.type',
  childSchemasJson___definitions___Workspace___title = 'childSchemasJson.definitions.Workspace.title',
  childSchemasJson___definitions___Workspace___description = 'childSchemasJson.definitions.Workspace.description',
  childSchemasJson___definitions___Workspace___type = 'childSchemasJson.definitions.Workspace.type',
  childSchemasJson___additionalProperties = 'childSchemasJson.additionalProperties'
}

type FileFilterInput = {
  readonly sourceInstanceName: Maybe<StringQueryOperatorInput>;
  readonly absolutePath: Maybe<StringQueryOperatorInput>;
  readonly relativePath: Maybe<StringQueryOperatorInput>;
  readonly extension: Maybe<StringQueryOperatorInput>;
  readonly size: Maybe<IntQueryOperatorInput>;
  readonly prettySize: Maybe<StringQueryOperatorInput>;
  readonly modifiedTime: Maybe<DateQueryOperatorInput>;
  readonly accessTime: Maybe<DateQueryOperatorInput>;
  readonly changeTime: Maybe<DateQueryOperatorInput>;
  readonly birthTime: Maybe<DateQueryOperatorInput>;
  readonly root: Maybe<StringQueryOperatorInput>;
  readonly dir: Maybe<StringQueryOperatorInput>;
  readonly base: Maybe<StringQueryOperatorInput>;
  readonly ext: Maybe<StringQueryOperatorInput>;
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly relativeDirectory: Maybe<StringQueryOperatorInput>;
  readonly dev: Maybe<IntQueryOperatorInput>;
  readonly mode: Maybe<IntQueryOperatorInput>;
  readonly nlink: Maybe<IntQueryOperatorInput>;
  readonly uid: Maybe<IntQueryOperatorInput>;
  readonly gid: Maybe<IntQueryOperatorInput>;
  readonly rdev: Maybe<IntQueryOperatorInput>;
  readonly ino: Maybe<FloatQueryOperatorInput>;
  readonly atimeMs: Maybe<FloatQueryOperatorInput>;
  readonly mtimeMs: Maybe<FloatQueryOperatorInput>;
  readonly ctimeMs: Maybe<FloatQueryOperatorInput>;
  readonly atime: Maybe<DateQueryOperatorInput>;
  readonly mtime: Maybe<DateQueryOperatorInput>;
  readonly ctime: Maybe<DateQueryOperatorInput>;
  readonly birthtime: Maybe<DateQueryOperatorInput>;
  readonly birthtimeMs: Maybe<FloatQueryOperatorInput>;
  readonly blksize: Maybe<IntQueryOperatorInput>;
  readonly blocks: Maybe<IntQueryOperatorInput>;
  readonly publicURL: Maybe<StringQueryOperatorInput>;
  readonly childImageSharp: Maybe<ImageSharpFilterInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
  readonly childMdx: Maybe<MdxFilterInput>;
  readonly childSchemasJson: Maybe<SchemasJsonFilterInput>;
};

type FileGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<FileEdge>;
  readonly nodes: ReadonlyArray<File>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type FileSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<FileFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type FloatQueryOperatorInput = {
  readonly eq: Maybe<Scalars['Float']>;
  readonly ne: Maybe<Scalars['Float']>;
  readonly gt: Maybe<Scalars['Float']>;
  readonly gte: Maybe<Scalars['Float']>;
  readonly lt: Maybe<Scalars['Float']>;
  readonly lte: Maybe<Scalars['Float']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['Float']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['Float']>>>;
};

enum HeadingsMdx {
  h1 = 'h1',
  h2 = 'h2',
  h3 = 'h3',
  h4 = 'h4',
  h5 = 'h5',
  h6 = 'h6'
}

enum ImageCropFocus {
  CENTER = 0,
  NORTH = 1,
  NORTHEAST = 5,
  EAST = 2,
  SOUTHEAST = 6,
  SOUTH = 3,
  SOUTHWEST = 7,
  WEST = 4,
  NORTHWEST = 8,
  ENTROPY = 16,
  ATTENTION = 17
}

enum ImageFit {
  COVER = 'cover',
  CONTAIN = 'contain',
  FILL = 'fill',
  INSIDE = 'inside',
  OUTSIDE = 'outside'
}

enum ImageFormat {
  NO_CHANGE = '',
  AUTO = '',
  JPG = 'jpg',
  PNG = 'png',
  WEBP = 'webp'
}

enum ImageLayout {
  FIXED = 'fixed',
  FLUID = 'fluid',
  CONSTRAINED = 'constrained'
}

enum ImagePlaceholder {
  DOMINANT_COLOR = 'dominantColor',
  TRACED_SVG = 'tracedSVG',
  BLURRED = 'blurred',
  NONE = 'none'
}

type ImageSharp = Node & {
  readonly fixed: Maybe<ImageSharpFixed>;
  /** @deprecated Resolutions was deprecated in Gatsby v2. It's been renamed to "fixed" https://example.com/write-docs-and-fix-this-example-link */
  readonly resolutions: Maybe<ImageSharpResolutions>;
  readonly fluid: Maybe<ImageSharpFluid>;
  /** @deprecated Sizes was deprecated in Gatsby v2. It's been renamed to "fluid" https://example.com/write-docs-and-fix-this-example-link */
  readonly sizes: Maybe<ImageSharpSizes>;
  readonly gatsbyImageData: Scalars['JSON'];
  readonly original: Maybe<ImageSharpOriginal>;
  readonly resize: Maybe<ImageSharpResize>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
};


type ImageSharp_fixedArgs = {
  width: Maybe<Scalars['Int']>;
  height: Maybe<Scalars['Int']>;
  base64Width: Maybe<Scalars['Int']>;
  jpegProgressive?: Maybe<Scalars['Boolean']>;
  pngCompressionSpeed?: Maybe<Scalars['Int']>;
  grayscale?: Maybe<Scalars['Boolean']>;
  duotone: Maybe<DuotoneGradient>;
  traceSVG: Maybe<Potrace>;
  quality: Maybe<Scalars['Int']>;
  jpegQuality: Maybe<Scalars['Int']>;
  pngQuality: Maybe<Scalars['Int']>;
  webpQuality: Maybe<Scalars['Int']>;
  toFormat?: Maybe<ImageFormat>;
  toFormatBase64?: Maybe<ImageFormat>;
  cropFocus?: Maybe<ImageCropFocus>;
  fit?: Maybe<ImageFit>;
  background?: Maybe<Scalars['String']>;
  rotate?: Maybe<Scalars['Int']>;
  trim?: Maybe<Scalars['Float']>;
};


type ImageSharp_resolutionsArgs = {
  width: Maybe<Scalars['Int']>;
  height: Maybe<Scalars['Int']>;
  base64Width: Maybe<Scalars['Int']>;
  jpegProgressive?: Maybe<Scalars['Boolean']>;
  pngCompressionSpeed?: Maybe<Scalars['Int']>;
  grayscale?: Maybe<Scalars['Boolean']>;
  duotone: Maybe<DuotoneGradient>;
  traceSVG: Maybe<Potrace>;
  quality: Maybe<Scalars['Int']>;
  jpegQuality: Maybe<Scalars['Int']>;
  pngQuality: Maybe<Scalars['Int']>;
  webpQuality: Maybe<Scalars['Int']>;
  toFormat?: Maybe<ImageFormat>;
  toFormatBase64?: Maybe<ImageFormat>;
  cropFocus?: Maybe<ImageCropFocus>;
  fit?: Maybe<ImageFit>;
  background?: Maybe<Scalars['String']>;
  rotate?: Maybe<Scalars['Int']>;
  trim?: Maybe<Scalars['Float']>;
};


type ImageSharp_fluidArgs = {
  maxWidth: Maybe<Scalars['Int']>;
  maxHeight: Maybe<Scalars['Int']>;
  base64Width: Maybe<Scalars['Int']>;
  grayscale?: Maybe<Scalars['Boolean']>;
  jpegProgressive?: Maybe<Scalars['Boolean']>;
  pngCompressionSpeed?: Maybe<Scalars['Int']>;
  duotone: Maybe<DuotoneGradient>;
  traceSVG: Maybe<Potrace>;
  quality: Maybe<Scalars['Int']>;
  jpegQuality: Maybe<Scalars['Int']>;
  pngQuality: Maybe<Scalars['Int']>;
  webpQuality: Maybe<Scalars['Int']>;
  toFormat?: Maybe<ImageFormat>;
  toFormatBase64?: Maybe<ImageFormat>;
  cropFocus?: Maybe<ImageCropFocus>;
  fit?: Maybe<ImageFit>;
  background?: Maybe<Scalars['String']>;
  rotate?: Maybe<Scalars['Int']>;
  trim?: Maybe<Scalars['Float']>;
  sizes?: Maybe<Scalars['String']>;
  srcSetBreakpoints?: Maybe<ReadonlyArray<Maybe<Scalars['Int']>>>;
};


type ImageSharp_sizesArgs = {
  maxWidth: Maybe<Scalars['Int']>;
  maxHeight: Maybe<Scalars['Int']>;
  base64Width: Maybe<Scalars['Int']>;
  grayscale?: Maybe<Scalars['Boolean']>;
  jpegProgressive?: Maybe<Scalars['Boolean']>;
  pngCompressionSpeed?: Maybe<Scalars['Int']>;
  duotone: Maybe<DuotoneGradient>;
  traceSVG: Maybe<Potrace>;
  quality: Maybe<Scalars['Int']>;
  jpegQuality: Maybe<Scalars['Int']>;
  pngQuality: Maybe<Scalars['Int']>;
  webpQuality: Maybe<Scalars['Int']>;
  toFormat?: Maybe<ImageFormat>;
  toFormatBase64?: Maybe<ImageFormat>;
  cropFocus?: Maybe<ImageCropFocus>;
  fit?: Maybe<ImageFit>;
  background?: Maybe<Scalars['String']>;
  rotate?: Maybe<Scalars['Int']>;
  trim?: Maybe<Scalars['Float']>;
  sizes?: Maybe<Scalars['String']>;
  srcSetBreakpoints?: Maybe<ReadonlyArray<Maybe<Scalars['Int']>>>;
};


type ImageSharp_gatsbyImageDataArgs = {
  layout?: Maybe<ImageLayout>;
  maxWidth: Maybe<Scalars['Int']>;
  maxHeight: Maybe<Scalars['Int']>;
  width: Maybe<Scalars['Int']>;
  height: Maybe<Scalars['Int']>;
  placeholder?: Maybe<ImagePlaceholder>;
  blurredOptions: Maybe<BlurredOptions>;
  tracedSVGOptions: Maybe<Potrace>;
  formats?: Maybe<ReadonlyArray<Maybe<ImageFormat>>>;
  outputPixelDensities: Maybe<ReadonlyArray<Maybe<Scalars['Float']>>>;
  sizes?: Maybe<Scalars['String']>;
  quality: Maybe<Scalars['Int']>;
  jpgOptions: Maybe<JPGOptions>;
  pngOptions: Maybe<PNGOptions>;
  webpOptions: Maybe<WebPOptions>;
  transformOptions: Maybe<TransformOptions>;
  background?: Maybe<Scalars['String']>;
};


type ImageSharp_resizeArgs = {
  width: Maybe<Scalars['Int']>;
  height: Maybe<Scalars['Int']>;
  quality: Maybe<Scalars['Int']>;
  jpegQuality: Maybe<Scalars['Int']>;
  pngQuality: Maybe<Scalars['Int']>;
  webpQuality: Maybe<Scalars['Int']>;
  jpegProgressive?: Maybe<Scalars['Boolean']>;
  pngCompressionLevel?: Maybe<Scalars['Int']>;
  pngCompressionSpeed?: Maybe<Scalars['Int']>;
  grayscale?: Maybe<Scalars['Boolean']>;
  duotone: Maybe<DuotoneGradient>;
  base64?: Maybe<Scalars['Boolean']>;
  traceSVG: Maybe<Potrace>;
  toFormat?: Maybe<ImageFormat>;
  cropFocus?: Maybe<ImageCropFocus>;
  fit?: Maybe<ImageFit>;
  background?: Maybe<Scalars['String']>;
  rotate?: Maybe<Scalars['Int']>;
  trim?: Maybe<Scalars['Float']>;
};

type ImageSharpConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<ImageSharpEdge>;
  readonly nodes: ReadonlyArray<ImageSharp>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<ImageSharpGroupConnection>;
};


type ImageSharpConnection_distinctArgs = {
  field: ImageSharpFieldsEnum;
};


type ImageSharpConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: ImageSharpFieldsEnum;
};

type ImageSharpEdge = {
  readonly next: Maybe<ImageSharp>;
  readonly node: ImageSharp;
  readonly previous: Maybe<ImageSharp>;
};

enum ImageSharpFieldsEnum {
  fixed___base64 = 'fixed.base64',
  fixed___tracedSVG = 'fixed.tracedSVG',
  fixed___aspectRatio = 'fixed.aspectRatio',
  fixed___width = 'fixed.width',
  fixed___height = 'fixed.height',
  fixed___src = 'fixed.src',
  fixed___srcSet = 'fixed.srcSet',
  fixed___srcWebp = 'fixed.srcWebp',
  fixed___srcSetWebp = 'fixed.srcSetWebp',
  fixed___originalName = 'fixed.originalName',
  resolutions___base64 = 'resolutions.base64',
  resolutions___tracedSVG = 'resolutions.tracedSVG',
  resolutions___aspectRatio = 'resolutions.aspectRatio',
  resolutions___width = 'resolutions.width',
  resolutions___height = 'resolutions.height',
  resolutions___src = 'resolutions.src',
  resolutions___srcSet = 'resolutions.srcSet',
  resolutions___srcWebp = 'resolutions.srcWebp',
  resolutions___srcSetWebp = 'resolutions.srcSetWebp',
  resolutions___originalName = 'resolutions.originalName',
  fluid___base64 = 'fluid.base64',
  fluid___tracedSVG = 'fluid.tracedSVG',
  fluid___aspectRatio = 'fluid.aspectRatio',
  fluid___src = 'fluid.src',
  fluid___srcSet = 'fluid.srcSet',
  fluid___srcWebp = 'fluid.srcWebp',
  fluid___srcSetWebp = 'fluid.srcSetWebp',
  fluid___sizes = 'fluid.sizes',
  fluid___originalImg = 'fluid.originalImg',
  fluid___originalName = 'fluid.originalName',
  fluid___presentationWidth = 'fluid.presentationWidth',
  fluid___presentationHeight = 'fluid.presentationHeight',
  sizes___base64 = 'sizes.base64',
  sizes___tracedSVG = 'sizes.tracedSVG',
  sizes___aspectRatio = 'sizes.aspectRatio',
  sizes___src = 'sizes.src',
  sizes___srcSet = 'sizes.srcSet',
  sizes___srcWebp = 'sizes.srcWebp',
  sizes___srcSetWebp = 'sizes.srcSetWebp',
  sizes___sizes = 'sizes.sizes',
  sizes___originalImg = 'sizes.originalImg',
  sizes___originalName = 'sizes.originalName',
  sizes___presentationWidth = 'sizes.presentationWidth',
  sizes___presentationHeight = 'sizes.presentationHeight',
  gatsbyImageData = 'gatsbyImageData',
  original___width = 'original.width',
  original___height = 'original.height',
  original___src = 'original.src',
  resize___src = 'resize.src',
  resize___tracedSVG = 'resize.tracedSVG',
  resize___width = 'resize.width',
  resize___height = 'resize.height',
  resize___aspectRatio = 'resize.aspectRatio',
  resize___originalName = 'resize.originalName',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type'
}

type ImageSharpFilterInput = {
  readonly fixed: Maybe<ImageSharpFixedFilterInput>;
  readonly resolutions: Maybe<ImageSharpResolutionsFilterInput>;
  readonly fluid: Maybe<ImageSharpFluidFilterInput>;
  readonly sizes: Maybe<ImageSharpSizesFilterInput>;
  readonly gatsbyImageData: Maybe<JSONQueryOperatorInput>;
  readonly original: Maybe<ImageSharpOriginalFilterInput>;
  readonly resize: Maybe<ImageSharpResizeFilterInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
};

type ImageSharpFixed = {
  readonly base64: Maybe<Scalars['String']>;
  readonly tracedSVG: Maybe<Scalars['String']>;
  readonly aspectRatio: Maybe<Scalars['Float']>;
  readonly width: Scalars['Float'];
  readonly height: Scalars['Float'];
  readonly src: Scalars['String'];
  readonly srcSet: Scalars['String'];
  readonly srcWebp: Maybe<Scalars['String']>;
  readonly srcSetWebp: Maybe<Scalars['String']>;
  readonly originalName: Maybe<Scalars['String']>;
};

type ImageSharpFixedFilterInput = {
  readonly base64: Maybe<StringQueryOperatorInput>;
  readonly tracedSVG: Maybe<StringQueryOperatorInput>;
  readonly aspectRatio: Maybe<FloatQueryOperatorInput>;
  readonly width: Maybe<FloatQueryOperatorInput>;
  readonly height: Maybe<FloatQueryOperatorInput>;
  readonly src: Maybe<StringQueryOperatorInput>;
  readonly srcSet: Maybe<StringQueryOperatorInput>;
  readonly srcWebp: Maybe<StringQueryOperatorInput>;
  readonly srcSetWebp: Maybe<StringQueryOperatorInput>;
  readonly originalName: Maybe<StringQueryOperatorInput>;
};

type ImageSharpFluid = {
  readonly base64: Maybe<Scalars['String']>;
  readonly tracedSVG: Maybe<Scalars['String']>;
  readonly aspectRatio: Scalars['Float'];
  readonly src: Scalars['String'];
  readonly srcSet: Scalars['String'];
  readonly srcWebp: Maybe<Scalars['String']>;
  readonly srcSetWebp: Maybe<Scalars['String']>;
  readonly sizes: Scalars['String'];
  readonly originalImg: Maybe<Scalars['String']>;
  readonly originalName: Maybe<Scalars['String']>;
  readonly presentationWidth: Scalars['Int'];
  readonly presentationHeight: Scalars['Int'];
};

type ImageSharpFluidFilterInput = {
  readonly base64: Maybe<StringQueryOperatorInput>;
  readonly tracedSVG: Maybe<StringQueryOperatorInput>;
  readonly aspectRatio: Maybe<FloatQueryOperatorInput>;
  readonly src: Maybe<StringQueryOperatorInput>;
  readonly srcSet: Maybe<StringQueryOperatorInput>;
  readonly srcWebp: Maybe<StringQueryOperatorInput>;
  readonly srcSetWebp: Maybe<StringQueryOperatorInput>;
  readonly sizes: Maybe<StringQueryOperatorInput>;
  readonly originalImg: Maybe<StringQueryOperatorInput>;
  readonly originalName: Maybe<StringQueryOperatorInput>;
  readonly presentationWidth: Maybe<IntQueryOperatorInput>;
  readonly presentationHeight: Maybe<IntQueryOperatorInput>;
};

type ImageSharpGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<ImageSharpEdge>;
  readonly nodes: ReadonlyArray<ImageSharp>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type ImageSharpOriginal = {
  readonly width: Maybe<Scalars['Float']>;
  readonly height: Maybe<Scalars['Float']>;
  readonly src: Maybe<Scalars['String']>;
};

type ImageSharpOriginalFilterInput = {
  readonly width: Maybe<FloatQueryOperatorInput>;
  readonly height: Maybe<FloatQueryOperatorInput>;
  readonly src: Maybe<StringQueryOperatorInput>;
};

type ImageSharpResize = {
  readonly src: Maybe<Scalars['String']>;
  readonly tracedSVG: Maybe<Scalars['String']>;
  readonly width: Maybe<Scalars['Int']>;
  readonly height: Maybe<Scalars['Int']>;
  readonly aspectRatio: Maybe<Scalars['Float']>;
  readonly originalName: Maybe<Scalars['String']>;
};

type ImageSharpResizeFilterInput = {
  readonly src: Maybe<StringQueryOperatorInput>;
  readonly tracedSVG: Maybe<StringQueryOperatorInput>;
  readonly width: Maybe<IntQueryOperatorInput>;
  readonly height: Maybe<IntQueryOperatorInput>;
  readonly aspectRatio: Maybe<FloatQueryOperatorInput>;
  readonly originalName: Maybe<StringQueryOperatorInput>;
};

type ImageSharpResolutions = {
  readonly base64: Maybe<Scalars['String']>;
  readonly tracedSVG: Maybe<Scalars['String']>;
  readonly aspectRatio: Maybe<Scalars['Float']>;
  readonly width: Scalars['Float'];
  readonly height: Scalars['Float'];
  readonly src: Scalars['String'];
  readonly srcSet: Scalars['String'];
  readonly srcWebp: Maybe<Scalars['String']>;
  readonly srcSetWebp: Maybe<Scalars['String']>;
  readonly originalName: Maybe<Scalars['String']>;
};

type ImageSharpResolutionsFilterInput = {
  readonly base64: Maybe<StringQueryOperatorInput>;
  readonly tracedSVG: Maybe<StringQueryOperatorInput>;
  readonly aspectRatio: Maybe<FloatQueryOperatorInput>;
  readonly width: Maybe<FloatQueryOperatorInput>;
  readonly height: Maybe<FloatQueryOperatorInput>;
  readonly src: Maybe<StringQueryOperatorInput>;
  readonly srcSet: Maybe<StringQueryOperatorInput>;
  readonly srcWebp: Maybe<StringQueryOperatorInput>;
  readonly srcSetWebp: Maybe<StringQueryOperatorInput>;
  readonly originalName: Maybe<StringQueryOperatorInput>;
};

type ImageSharpSizes = {
  readonly base64: Maybe<Scalars['String']>;
  readonly tracedSVG: Maybe<Scalars['String']>;
  readonly aspectRatio: Scalars['Float'];
  readonly src: Scalars['String'];
  readonly srcSet: Scalars['String'];
  readonly srcWebp: Maybe<Scalars['String']>;
  readonly srcSetWebp: Maybe<Scalars['String']>;
  readonly sizes: Scalars['String'];
  readonly originalImg: Maybe<Scalars['String']>;
  readonly originalName: Maybe<Scalars['String']>;
  readonly presentationWidth: Scalars['Int'];
  readonly presentationHeight: Scalars['Int'];
};

type ImageSharpSizesFilterInput = {
  readonly base64: Maybe<StringQueryOperatorInput>;
  readonly tracedSVG: Maybe<StringQueryOperatorInput>;
  readonly aspectRatio: Maybe<FloatQueryOperatorInput>;
  readonly src: Maybe<StringQueryOperatorInput>;
  readonly srcSet: Maybe<StringQueryOperatorInput>;
  readonly srcWebp: Maybe<StringQueryOperatorInput>;
  readonly srcSetWebp: Maybe<StringQueryOperatorInput>;
  readonly sizes: Maybe<StringQueryOperatorInput>;
  readonly originalImg: Maybe<StringQueryOperatorInput>;
  readonly originalName: Maybe<StringQueryOperatorInput>;
  readonly presentationWidth: Maybe<IntQueryOperatorInput>;
  readonly presentationHeight: Maybe<IntQueryOperatorInput>;
};

type ImageSharpSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<ImageSharpFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type Internal = {
  readonly content: Maybe<Scalars['String']>;
  readonly contentDigest: Scalars['String'];
  readonly description: Maybe<Scalars['String']>;
  readonly fieldOwners: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly ignoreType: Maybe<Scalars['Boolean']>;
  readonly mediaType: Maybe<Scalars['String']>;
  readonly owner: Scalars['String'];
  readonly type: Scalars['String'];
};

type InternalFilterInput = {
  readonly content: Maybe<StringQueryOperatorInput>;
  readonly contentDigest: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly fieldOwners: Maybe<StringQueryOperatorInput>;
  readonly ignoreType: Maybe<BooleanQueryOperatorInput>;
  readonly mediaType: Maybe<StringQueryOperatorInput>;
  readonly owner: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
};

type IntQueryOperatorInput = {
  readonly eq: Maybe<Scalars['Int']>;
  readonly ne: Maybe<Scalars['Int']>;
  readonly gt: Maybe<Scalars['Int']>;
  readonly gte: Maybe<Scalars['Int']>;
  readonly lt: Maybe<Scalars['Int']>;
  readonly lte: Maybe<Scalars['Int']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['Int']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['Int']>>>;
};

type JPGOptions = {
  readonly quality: Maybe<Scalars['Int']>;
  readonly progressive: Maybe<Scalars['Boolean']>;
};


type JSONQueryOperatorInput = {
  readonly eq: Maybe<Scalars['JSON']>;
  readonly ne: Maybe<Scalars['JSON']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['JSON']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['JSON']>>>;
  readonly regex: Maybe<Scalars['JSON']>;
  readonly glob: Maybe<Scalars['JSON']>;
};

type Mdx = Node & {
  readonly rawBody: Scalars['String'];
  readonly fileAbsolutePath: Scalars['String'];
  readonly frontmatter: Maybe<MdxFrontmatter>;
  readonly slug: Maybe<Scalars['String']>;
  readonly body: Scalars['String'];
  readonly excerpt: Scalars['String'];
  readonly headings: Maybe<ReadonlyArray<Maybe<MdxHeadingMdx>>>;
  readonly html: Maybe<Scalars['String']>;
  readonly mdxAST: Maybe<Scalars['JSON']>;
  readonly tableOfContents: Maybe<Scalars['JSON']>;
  readonly timeToRead: Maybe<Scalars['Int']>;
  readonly wordCount: Maybe<MdxWordCount>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
};


type Mdx_excerptArgs = {
  pruneLength?: Maybe<Scalars['Int']>;
  truncate?: Maybe<Scalars['Boolean']>;
};


type Mdx_headingsArgs = {
  depth: Maybe<HeadingsMdx>;
};


type Mdx_tableOfContentsArgs = {
  maxDepth: Maybe<Scalars['Int']>;
};

type MdxConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<MdxEdge>;
  readonly nodes: ReadonlyArray<Mdx>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<MdxGroupConnection>;
};


type MdxConnection_distinctArgs = {
  field: MdxFieldsEnum;
};


type MdxConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: MdxFieldsEnum;
};

type MdxEdge = {
  readonly next: Maybe<Mdx>;
  readonly node: Mdx;
  readonly previous: Maybe<Mdx>;
};

enum MdxFieldsEnum {
  rawBody = 'rawBody',
  fileAbsolutePath = 'fileAbsolutePath',
  frontmatter___title = 'frontmatter.title',
  frontmatter___nav = 'frontmatter.nav',
  frontmatter___navOrder = 'frontmatter.navOrder',
  slug = 'slug',
  body = 'body',
  excerpt = 'excerpt',
  headings = 'headings',
  headings___value = 'headings.value',
  headings___depth = 'headings.depth',
  html = 'html',
  mdxAST = 'mdxAST',
  tableOfContents = 'tableOfContents',
  timeToRead = 'timeToRead',
  wordCount___paragraphs = 'wordCount.paragraphs',
  wordCount___sentences = 'wordCount.sentences',
  wordCount___words = 'wordCount.words',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type'
}

type MdxFilterInput = {
  readonly rawBody: Maybe<StringQueryOperatorInput>;
  readonly fileAbsolutePath: Maybe<StringQueryOperatorInput>;
  readonly frontmatter: Maybe<MdxFrontmatterFilterInput>;
  readonly slug: Maybe<StringQueryOperatorInput>;
  readonly body: Maybe<StringQueryOperatorInput>;
  readonly excerpt: Maybe<StringQueryOperatorInput>;
  readonly headings: Maybe<MdxHeadingMdxFilterListInput>;
  readonly html: Maybe<StringQueryOperatorInput>;
  readonly mdxAST: Maybe<JSONQueryOperatorInput>;
  readonly tableOfContents: Maybe<JSONQueryOperatorInput>;
  readonly timeToRead: Maybe<IntQueryOperatorInput>;
  readonly wordCount: Maybe<MdxWordCountFilterInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
};

type MdxFrontmatter = {
  readonly title: Scalars['String'];
  readonly nav: Maybe<Scalars['String']>;
  readonly navOrder: Maybe<Scalars['Int']>;
};

type MdxFrontmatterFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly nav: Maybe<StringQueryOperatorInput>;
  readonly navOrder: Maybe<IntQueryOperatorInput>;
};

type MdxGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<MdxEdge>;
  readonly nodes: ReadonlyArray<Mdx>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type MdxHeadingMdx = {
  readonly value: Maybe<Scalars['String']>;
  readonly depth: Maybe<Scalars['Int']>;
};

type MdxHeadingMdxFilterInput = {
  readonly value: Maybe<StringQueryOperatorInput>;
  readonly depth: Maybe<IntQueryOperatorInput>;
};

type MdxHeadingMdxFilterListInput = {
  readonly elemMatch: Maybe<MdxHeadingMdxFilterInput>;
};

type MdxSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<MdxFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type MdxWordCount = {
  readonly paragraphs: Maybe<Scalars['Int']>;
  readonly sentences: Maybe<Scalars['Int']>;
  readonly words: Maybe<Scalars['Int']>;
};

type MdxWordCountFilterInput = {
  readonly paragraphs: Maybe<IntQueryOperatorInput>;
  readonly sentences: Maybe<IntQueryOperatorInput>;
  readonly words: Maybe<IntQueryOperatorInput>;
};

/** Node Interface */
type Node = {
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
};

type NodeFilterInput = {
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
};

type NodeFilterListInput = {
  readonly elemMatch: Maybe<NodeFilterInput>;
};

type PageInfo = {
  readonly currentPage: Scalars['Int'];
  readonly hasPreviousPage: Scalars['Boolean'];
  readonly hasNextPage: Scalars['Boolean'];
  readonly itemCount: Scalars['Int'];
  readonly pageCount: Scalars['Int'];
  readonly perPage: Maybe<Scalars['Int']>;
  readonly totalCount: Scalars['Int'];
};

type PNGOptions = {
  readonly quality: Maybe<Scalars['Int']>;
  readonly compressionSpeed: Maybe<Scalars['Int']>;
};

type Potrace = {
  readonly turnPolicy: Maybe<PotraceTurnPolicy>;
  readonly turdSize: Maybe<Scalars['Float']>;
  readonly alphaMax: Maybe<Scalars['Float']>;
  readonly optCurve: Maybe<Scalars['Boolean']>;
  readonly optTolerance: Maybe<Scalars['Float']>;
  readonly threshold: Maybe<Scalars['Int']>;
  readonly blackOnWhite: Maybe<Scalars['Boolean']>;
  readonly color: Maybe<Scalars['String']>;
  readonly background: Maybe<Scalars['String']>;
};

enum PotraceTurnPolicy {
  TURNPOLICY_BLACK = 'black',
  TURNPOLICY_WHITE = 'white',
  TURNPOLICY_LEFT = 'left',
  TURNPOLICY_RIGHT = 'right',
  TURNPOLICY_MINORITY = 'minority',
  TURNPOLICY_MAJORITY = 'majority'
}

type Query = {
  readonly file: Maybe<File>;
  readonly allFile: FileConnection;
  readonly directory: Maybe<Directory>;
  readonly allDirectory: DirectoryConnection;
  readonly site: Maybe<Site>;
  readonly allSite: SiteConnection;
  readonly sitePage: Maybe<SitePage>;
  readonly allSitePage: SitePageConnection;
  readonly imageSharp: Maybe<ImageSharp>;
  readonly allImageSharp: ImageSharpConnection;
  readonly mdx: Maybe<Mdx>;
  readonly allMdx: MdxConnection;
  readonly schemasJson: Maybe<SchemasJson>;
  readonly allSchemasJson: SchemasJsonConnection;
  readonly siteBuildMetadata: Maybe<SiteBuildMetadata>;
  readonly allSiteBuildMetadata: SiteBuildMetadataConnection;
  readonly sitePlugin: Maybe<SitePlugin>;
  readonly allSitePlugin: SitePluginConnection;
};


type Query_fileArgs = {
  sourceInstanceName: Maybe<StringQueryOperatorInput>;
  absolutePath: Maybe<StringQueryOperatorInput>;
  relativePath: Maybe<StringQueryOperatorInput>;
  extension: Maybe<StringQueryOperatorInput>;
  size: Maybe<IntQueryOperatorInput>;
  prettySize: Maybe<StringQueryOperatorInput>;
  modifiedTime: Maybe<DateQueryOperatorInput>;
  accessTime: Maybe<DateQueryOperatorInput>;
  changeTime: Maybe<DateQueryOperatorInput>;
  birthTime: Maybe<DateQueryOperatorInput>;
  root: Maybe<StringQueryOperatorInput>;
  dir: Maybe<StringQueryOperatorInput>;
  base: Maybe<StringQueryOperatorInput>;
  ext: Maybe<StringQueryOperatorInput>;
  name: Maybe<StringQueryOperatorInput>;
  relativeDirectory: Maybe<StringQueryOperatorInput>;
  dev: Maybe<IntQueryOperatorInput>;
  mode: Maybe<IntQueryOperatorInput>;
  nlink: Maybe<IntQueryOperatorInput>;
  uid: Maybe<IntQueryOperatorInput>;
  gid: Maybe<IntQueryOperatorInput>;
  rdev: Maybe<IntQueryOperatorInput>;
  ino: Maybe<FloatQueryOperatorInput>;
  atimeMs: Maybe<FloatQueryOperatorInput>;
  mtimeMs: Maybe<FloatQueryOperatorInput>;
  ctimeMs: Maybe<FloatQueryOperatorInput>;
  atime: Maybe<DateQueryOperatorInput>;
  mtime: Maybe<DateQueryOperatorInput>;
  ctime: Maybe<DateQueryOperatorInput>;
  birthtime: Maybe<DateQueryOperatorInput>;
  birthtimeMs: Maybe<FloatQueryOperatorInput>;
  blksize: Maybe<IntQueryOperatorInput>;
  blocks: Maybe<IntQueryOperatorInput>;
  publicURL: Maybe<StringQueryOperatorInput>;
  childImageSharp: Maybe<ImageSharpFilterInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
  childMdx: Maybe<MdxFilterInput>;
  childSchemasJson: Maybe<SchemasJsonFilterInput>;
};


type Query_allFileArgs = {
  filter: Maybe<FileFilterInput>;
  sort: Maybe<FileSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_directoryArgs = {
  sourceInstanceName: Maybe<StringQueryOperatorInput>;
  absolutePath: Maybe<StringQueryOperatorInput>;
  relativePath: Maybe<StringQueryOperatorInput>;
  extension: Maybe<StringQueryOperatorInput>;
  size: Maybe<IntQueryOperatorInput>;
  prettySize: Maybe<StringQueryOperatorInput>;
  modifiedTime: Maybe<DateQueryOperatorInput>;
  accessTime: Maybe<DateQueryOperatorInput>;
  changeTime: Maybe<DateQueryOperatorInput>;
  birthTime: Maybe<DateQueryOperatorInput>;
  root: Maybe<StringQueryOperatorInput>;
  dir: Maybe<StringQueryOperatorInput>;
  base: Maybe<StringQueryOperatorInput>;
  ext: Maybe<StringQueryOperatorInput>;
  name: Maybe<StringQueryOperatorInput>;
  relativeDirectory: Maybe<StringQueryOperatorInput>;
  dev: Maybe<IntQueryOperatorInput>;
  mode: Maybe<IntQueryOperatorInput>;
  nlink: Maybe<IntQueryOperatorInput>;
  uid: Maybe<IntQueryOperatorInput>;
  gid: Maybe<IntQueryOperatorInput>;
  rdev: Maybe<IntQueryOperatorInput>;
  ino: Maybe<FloatQueryOperatorInput>;
  atimeMs: Maybe<FloatQueryOperatorInput>;
  mtimeMs: Maybe<FloatQueryOperatorInput>;
  ctimeMs: Maybe<FloatQueryOperatorInput>;
  atime: Maybe<DateQueryOperatorInput>;
  mtime: Maybe<DateQueryOperatorInput>;
  ctime: Maybe<DateQueryOperatorInput>;
  birthtime: Maybe<DateQueryOperatorInput>;
  birthtimeMs: Maybe<FloatQueryOperatorInput>;
  blksize: Maybe<IntQueryOperatorInput>;
  blocks: Maybe<IntQueryOperatorInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
};


type Query_allDirectoryArgs = {
  filter: Maybe<DirectoryFilterInput>;
  sort: Maybe<DirectorySortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_siteArgs = {
  buildTime: Maybe<DateQueryOperatorInput>;
  siteMetadata: Maybe<SiteSiteMetadataFilterInput>;
  polyfill: Maybe<BooleanQueryOperatorInput>;
  pathPrefix: Maybe<StringQueryOperatorInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
};


type Query_allSiteArgs = {
  filter: Maybe<SiteFilterInput>;
  sort: Maybe<SiteSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_sitePageArgs = {
  path: Maybe<StringQueryOperatorInput>;
  component: Maybe<StringQueryOperatorInput>;
  internalComponentName: Maybe<StringQueryOperatorInput>;
  componentChunkName: Maybe<StringQueryOperatorInput>;
  matchPath: Maybe<StringQueryOperatorInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
  isCreatedByStatefulCreatePages: Maybe<BooleanQueryOperatorInput>;
  context: Maybe<SitePageContextFilterInput>;
  pluginCreator: Maybe<SitePluginFilterInput>;
  pluginCreatorId: Maybe<StringQueryOperatorInput>;
  componentPath: Maybe<StringQueryOperatorInput>;
};


type Query_allSitePageArgs = {
  filter: Maybe<SitePageFilterInput>;
  sort: Maybe<SitePageSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_imageSharpArgs = {
  fixed: Maybe<ImageSharpFixedFilterInput>;
  resolutions: Maybe<ImageSharpResolutionsFilterInput>;
  fluid: Maybe<ImageSharpFluidFilterInput>;
  sizes: Maybe<ImageSharpSizesFilterInput>;
  gatsbyImageData: Maybe<JSONQueryOperatorInput>;
  original: Maybe<ImageSharpOriginalFilterInput>;
  resize: Maybe<ImageSharpResizeFilterInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
};


type Query_allImageSharpArgs = {
  filter: Maybe<ImageSharpFilterInput>;
  sort: Maybe<ImageSharpSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_mdxArgs = {
  rawBody: Maybe<StringQueryOperatorInput>;
  fileAbsolutePath: Maybe<StringQueryOperatorInput>;
  frontmatter: Maybe<MdxFrontmatterFilterInput>;
  slug: Maybe<StringQueryOperatorInput>;
  body: Maybe<StringQueryOperatorInput>;
  excerpt: Maybe<StringQueryOperatorInput>;
  headings: Maybe<MdxHeadingMdxFilterListInput>;
  html: Maybe<StringQueryOperatorInput>;
  mdxAST: Maybe<JSONQueryOperatorInput>;
  tableOfContents: Maybe<JSONQueryOperatorInput>;
  timeToRead: Maybe<IntQueryOperatorInput>;
  wordCount: Maybe<MdxWordCountFilterInput>;
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
};


type Query_allMdxArgs = {
  filter: Maybe<MdxFilterInput>;
  sort: Maybe<MdxSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_schemasJsonArgs = {
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
  _schema: Maybe<StringQueryOperatorInput>;
  title: Maybe<StringQueryOperatorInput>;
  type: Maybe<StringQueryOperatorInput>;
  description: Maybe<StringQueryOperatorInput>;
  x_taplo_info: Maybe<SchemasJsonX_taplo_infoFilterInput>;
  properties: Maybe<SchemasJsonPropertiesFilterInput>;
  definitions: Maybe<SchemasJsonDefinitionsFilterInput>;
  additionalProperties: Maybe<BooleanQueryOperatorInput>;
};


type Query_allSchemasJsonArgs = {
  filter: Maybe<SchemasJsonFilterInput>;
  sort: Maybe<SchemasJsonSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_siteBuildMetadataArgs = {
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
  buildTime: Maybe<DateQueryOperatorInput>;
};


type Query_allSiteBuildMetadataArgs = {
  filter: Maybe<SiteBuildMetadataFilterInput>;
  sort: Maybe<SiteBuildMetadataSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};


type Query_sitePluginArgs = {
  id: Maybe<StringQueryOperatorInput>;
  parent: Maybe<NodeFilterInput>;
  children: Maybe<NodeFilterListInput>;
  internal: Maybe<InternalFilterInput>;
  resolve: Maybe<StringQueryOperatorInput>;
  name: Maybe<StringQueryOperatorInput>;
  version: Maybe<StringQueryOperatorInput>;
  pluginOptions: Maybe<SitePluginPluginOptionsFilterInput>;
  nodeAPIs: Maybe<StringQueryOperatorInput>;
  browserAPIs: Maybe<StringQueryOperatorInput>;
  ssrAPIs: Maybe<StringQueryOperatorInput>;
  pluginFilepath: Maybe<StringQueryOperatorInput>;
  packageJson: Maybe<SitePluginPackageJsonFilterInput>;
};


type Query_allSitePluginArgs = {
  filter: Maybe<SitePluginFilterInput>;
  sort: Maybe<SitePluginSortInput>;
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
};

type SchemasJson = Node & {
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
  readonly _schema: Maybe<Scalars['String']>;
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly x_taplo_info: Maybe<SchemasJsonX_taplo_info>;
  readonly properties: Maybe<SchemasJsonProperties>;
  readonly definitions: Maybe<SchemasJsonDefinitions>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
};

type SchemasJsonConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SchemasJsonEdge>;
  readonly nodes: ReadonlyArray<SchemasJson>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<SchemasJsonGroupConnection>;
};


type SchemasJsonConnection_distinctArgs = {
  field: SchemasJsonFieldsEnum;
};


type SchemasJsonConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: SchemasJsonFieldsEnum;
};

type SchemasJsonDefinitions = {
  readonly poetry_author_pattern: Maybe<SchemasJsonDefinitionsPoetry_author_pattern>;
  readonly poetry_authors: Maybe<SchemasJsonDefinitionsPoetry_authors>;
  readonly poetry_maintainers: Maybe<SchemasJsonDefinitionsPoetry_maintainers>;
  readonly poetry_dependency_any: Maybe<SchemasJsonDefinitionsPoetry_dependency_any>;
  readonly poetry_pep440_version: Maybe<SchemasJsonDefinitionsPoetry_pep440_version>;
  readonly poetry_dependency: Maybe<SchemasJsonDefinitionsPoetry_dependency>;
  readonly poetry_long_dependency: Maybe<SchemasJsonDefinitionsPoetry_long_dependency>;
  readonly poetry_git_dependency: Maybe<SchemasJsonDefinitionsPoetry_git_dependency>;
  readonly poetry_file_dependency: Maybe<SchemasJsonDefinitionsPoetry_file_dependency>;
  readonly poetry_path_dependency: Maybe<SchemasJsonDefinitionsPoetry_path_dependency>;
  readonly poetry_url_dependency: Maybe<SchemasJsonDefinitionsPoetry_url_dependency>;
  readonly poetry_multiple_constraints_dependency: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependency>;
  readonly poetry_scripts: Maybe<SchemasJsonDefinitionsPoetry_scripts>;
  readonly poetry_script: Maybe<SchemasJsonDefinitionsPoetry_script>;
  readonly poetry_extra_script: Maybe<SchemasJsonDefinitionsPoetry_extra_script>;
  readonly poetry_repository: Maybe<SchemasJsonDefinitionsPoetry_repository>;
  readonly BuildSystem: Maybe<SchemasJsonDefinitionsBuildSystem>;
  readonly Build: Maybe<SchemasJsonDefinitionsBuild>;
  readonly DebugLevel: Maybe<SchemasJsonDefinitionsDebugLevel>;
  readonly Dependency: Maybe<SchemasJsonDefinitionsDependency>;
  readonly DetailedDependency: Maybe<SchemasJsonDefinitionsDetailedDependency>;
  readonly Edition: Maybe<SchemasJsonDefinitionsEdition>;
  readonly Lto: Maybe<SchemasJsonDefinitionsLto>;
  readonly MetaBuild: Maybe<SchemasJsonDefinitionsMetaBuild>;
  readonly OptLevel: Maybe<SchemasJsonDefinitionsOptLevel>;
  readonly Package: Maybe<SchemasJsonDefinitionsPackage>;
  readonly Panic: Maybe<SchemasJsonDefinitionsPanic>;
  readonly Platform: Maybe<SchemasJsonDefinitionsPlatform>;
  readonly Profile: Maybe<SchemasJsonDefinitionsProfile>;
  readonly Profiles: Maybe<SchemasJsonDefinitionsProfiles>;
  readonly Publish: Maybe<SchemasJsonDefinitionsPublish>;
  readonly Readme: Maybe<SchemasJsonDefinitionsReadme>;
  readonly SemVer: Maybe<SchemasJsonDefinitionsSemVer>;
  readonly SemVerRequirement: Maybe<SchemasJsonDefinitionsSemVerRequirement>;
  readonly Target: Maybe<SchemasJsonDefinitionsTarget>;
  readonly Workspace: Maybe<SchemasJsonDefinitionsWorkspace>;
};

type SchemasJsonDefinitionsBuild = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsBuildAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildX_taplo>;
};

type SchemasJsonDefinitionsBuildAnyOf = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsBuildAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsBuildAnyOfFilterInput>;
};

type SchemasJsonDefinitionsBuildFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsBuildAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildX_taploFilterInput>;
};

type SchemasJsonDefinitionsBuildSystem = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemX_taplo>;
  readonly description: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly properties: Maybe<SchemasJsonDefinitionsBuildSystemProperties>;
};

type SchemasJsonDefinitionsBuildSystemFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemX_taploFilterInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemProperties = {
  readonly requires: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequires>;
  readonly build_backend: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backend>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backend = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taplo>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploLinks>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesFilterInput = {
  readonly requires: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresFilterInput>;
  readonly build_backend: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesBuild_backendFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequires = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taplo>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploLinks>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildSystemPropertiesRequiresX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsBuildSystemX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemX_taploLinks>;
};

type SchemasJsonDefinitionsBuildSystemX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildSystemX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsBuildSystemX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildSystemX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsBuildX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildX_taploLinks>;
};

type SchemasJsonDefinitionsBuildX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsBuildX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsBuildX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsBuildX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDebugLevel = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsDebugLevelAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDebugLevelX_taplo>;
};

type SchemasJsonDefinitionsDebugLevelAnyOf = {
  readonly type: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
  readonly minimum: Maybe<Scalars['Int']>;
};

type SchemasJsonDefinitionsDebugLevelAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
  readonly minimum: Maybe<IntQueryOperatorInput>;
};

type SchemasJsonDefinitionsDebugLevelAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsDebugLevelAnyOfFilterInput>;
};

type SchemasJsonDefinitionsDebugLevelFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsDebugLevelAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDebugLevelX_taploFilterInput>;
};

type SchemasJsonDefinitionsDebugLevelX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsDebugLevelX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsDebugLevelX_taploLinks>;
};

type SchemasJsonDefinitionsDebugLevelX_taploDocs = {
  readonly enumValues: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonDefinitionsDebugLevelX_taploDocsFilterInput = {
  readonly enumValues: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDebugLevelX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsDebugLevelX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsDebugLevelX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDebugLevelX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDebugLevelX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDependency = {
  readonly title: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsDependencyAnyOf>>>;
};

type SchemasJsonDefinitionsDependencyAnyOf = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDependencyAnyOfFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDependencyAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsDependencyAnyOfFilterInput>;
};

type SchemasJsonDefinitionsDependencyFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsDependencyAnyOfFilterListInput>;
};

type SchemasJsonDefinitionsDetailedDependency = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsDetailedDependencyProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyProperties = {
  readonly branch: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranch>;
  readonly default_features: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_features>;
  readonly features: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeatures>;
  readonly git: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGit>;
  readonly optional: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptional>;
  readonly package: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackage>;
  readonly path: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPath>;
  readonly public: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPublic>;
  readonly registry: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry>;
  readonly registry_index: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_index>;
  readonly rev: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRev>;
  readonly tag: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTag>;
  readonly version: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesVersion>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranch = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranchFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesBranchX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_features = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeatures = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesFilterInput = {
  readonly branch: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesBranchFilterInput>;
  readonly default_features: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesDefault_featuresFilterInput>;
  readonly features: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesFeaturesFilterInput>;
  readonly git: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGitFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalFilterInput>;
  readonly package: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackageFilterInput>;
  readonly path: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPathFilterInput>;
  readonly public: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPublicFilterInput>;
  readonly registry: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryFilterInput>;
  readonly registry_index: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexFilterInput>;
  readonly rev: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRevFilterInput>;
  readonly tag: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTagFilterInput>;
  readonly version: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesVersionFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGit = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGitFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesGitX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptional = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesOptionalX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackage = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackageFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPackageX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPath = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPathFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPathX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPublic = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPublicX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPublicFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesPublicX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPublicX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesPublicX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_index = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistry_indexX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRegistryX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRev = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRevFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesRevX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTag = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taplo>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTagFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploLinks>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesTagX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesVersion = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsDetailedDependencyPropertiesVersionFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsDetailedDependencyX_taplo = {
  readonly initFields: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonDefinitionsDetailedDependencyX_taploFilterInput = {
  readonly initFields: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsEdition = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Date']>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsEditionX_taplo>;
};


type SchemasJsonDefinitionsEdition_enumArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsEditionFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<DateQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsEditionX_taploFilterInput>;
};

type SchemasJsonDefinitionsEditionX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsEditionX_taploLinks>;
};

type SchemasJsonDefinitionsEditionX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsEditionX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsEditionX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsEditionX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsFilterInput = {
  readonly poetry_author_pattern: Maybe<SchemasJsonDefinitionsPoetry_author_patternFilterInput>;
  readonly poetry_authors: Maybe<SchemasJsonDefinitionsPoetry_authorsFilterInput>;
  readonly poetry_maintainers: Maybe<SchemasJsonDefinitionsPoetry_maintainersFilterInput>;
  readonly poetry_dependency_any: Maybe<SchemasJsonDefinitionsPoetry_dependency_anyFilterInput>;
  readonly poetry_pep440_version: Maybe<SchemasJsonDefinitionsPoetry_pep440_versionFilterInput>;
  readonly poetry_dependency: Maybe<SchemasJsonDefinitionsPoetry_dependencyFilterInput>;
  readonly poetry_long_dependency: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyFilterInput>;
  readonly poetry_git_dependency: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyFilterInput>;
  readonly poetry_file_dependency: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyFilterInput>;
  readonly poetry_path_dependency: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyFilterInput>;
  readonly poetry_url_dependency: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyFilterInput>;
  readonly poetry_multiple_constraints_dependency: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyFilterInput>;
  readonly poetry_scripts: Maybe<SchemasJsonDefinitionsPoetry_scriptsFilterInput>;
  readonly poetry_script: Maybe<SchemasJsonDefinitionsPoetry_scriptFilterInput>;
  readonly poetry_extra_script: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptFilterInput>;
  readonly poetry_repository: Maybe<SchemasJsonDefinitionsPoetry_repositoryFilterInput>;
  readonly BuildSystem: Maybe<SchemasJsonDefinitionsBuildSystemFilterInput>;
  readonly Build: Maybe<SchemasJsonDefinitionsBuildFilterInput>;
  readonly DebugLevel: Maybe<SchemasJsonDefinitionsDebugLevelFilterInput>;
  readonly Dependency: Maybe<SchemasJsonDefinitionsDependencyFilterInput>;
  readonly DetailedDependency: Maybe<SchemasJsonDefinitionsDetailedDependencyFilterInput>;
  readonly Edition: Maybe<SchemasJsonDefinitionsEditionFilterInput>;
  readonly Lto: Maybe<SchemasJsonDefinitionsLtoFilterInput>;
  readonly MetaBuild: Maybe<SchemasJsonDefinitionsMetaBuildFilterInput>;
  readonly OptLevel: Maybe<SchemasJsonDefinitionsOptLevelFilterInput>;
  readonly Package: Maybe<SchemasJsonDefinitionsPackageFilterInput>;
  readonly Panic: Maybe<SchemasJsonDefinitionsPanicFilterInput>;
  readonly Platform: Maybe<SchemasJsonDefinitionsPlatformFilterInput>;
  readonly Profile: Maybe<SchemasJsonDefinitionsProfileFilterInput>;
  readonly Profiles: Maybe<SchemasJsonDefinitionsProfilesFilterInput>;
  readonly Publish: Maybe<SchemasJsonDefinitionsPublishFilterInput>;
  readonly Readme: Maybe<SchemasJsonDefinitionsReadmeFilterInput>;
  readonly SemVer: Maybe<SchemasJsonDefinitionsSemVerFilterInput>;
  readonly SemVerRequirement: Maybe<SchemasJsonDefinitionsSemVerRequirementFilterInput>;
  readonly Target: Maybe<SchemasJsonDefinitionsTargetFilterInput>;
  readonly Workspace: Maybe<SchemasJsonDefinitionsWorkspaceFilterInput>;
};

type SchemasJsonDefinitionsLto = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsLtoAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsLtoX_taplo>;
};

type SchemasJsonDefinitionsLtoAnyOf = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsLtoAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsLtoAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsLtoAnyOfFilterInput>;
};

type SchemasJsonDefinitionsLtoFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsLtoAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsLtoX_taploFilterInput>;
};

type SchemasJsonDefinitionsLtoX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsLtoX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsLtoX_taploLinks>;
};

type SchemasJsonDefinitionsLtoX_taploDocs = {
  readonly enumValues: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonDefinitionsLtoX_taploDocsFilterInput = {
  readonly enumValues: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsLtoX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsLtoX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsLtoX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsLtoX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsLtoX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsMetaBuild = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsMetaBuildItems>;
};

type SchemasJsonDefinitionsMetaBuildFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsMetaBuildItemsFilterInput>;
};

type SchemasJsonDefinitionsMetaBuildItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsMetaBuildItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsOptLevel = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsOptLevelAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsOptLevelX_taplo>;
};

type SchemasJsonDefinitionsOptLevelAnyOf = {
  readonly type: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsOptLevelAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsOptLevelAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsOptLevelAnyOfFilterInput>;
};

type SchemasJsonDefinitionsOptLevelFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsOptLevelAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsOptLevelX_taploFilterInput>;
};

type SchemasJsonDefinitionsOptLevelX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsOptLevelX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsOptLevelX_taploLinks>;
};

type SchemasJsonDefinitionsOptLevelX_taploDocs = {
  readonly enumValues: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonDefinitionsOptLevelX_taploDocsFilterInput = {
  readonly enumValues: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsOptLevelX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsOptLevelX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsOptLevelX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsOptLevelX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsOptLevelX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackage = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly properties: Maybe<SchemasJsonDefinitionsPackageProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackageX_taplo>;
};

type SchemasJsonDefinitionsPackageFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPackagePropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackageX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackageProperties = {
  readonly authors: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthors>;
  readonly autobenches: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenches>;
  readonly autobins: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobins>;
  readonly autoexamples: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamples>;
  readonly autotests: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotests>;
  readonly build: Maybe<SchemasJsonDefinitionsPackagePropertiesBuild>;
  readonly categories: Maybe<SchemasJsonDefinitionsPackagePropertiesCategories>;
  readonly default_run: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_run>;
  readonly description: Maybe<SchemasJsonDefinitionsPackagePropertiesDescription>;
  readonly documentation: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentation>;
  readonly edition: Maybe<SchemasJsonDefinitionsPackagePropertiesEdition>;
  readonly exclude: Maybe<SchemasJsonDefinitionsPackagePropertiesExclude>;
  readonly homepage: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepage>;
  readonly im_a_teapot: Maybe<SchemasJsonDefinitionsPackagePropertiesIm_a_teapot>;
  readonly include: Maybe<SchemasJsonDefinitionsPackagePropertiesInclude>;
  readonly keywords: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywords>;
  readonly license: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense>;
  readonly license_file: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_file>;
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLinks>;
  readonly metabuild: Maybe<SchemasJsonDefinitionsPackagePropertiesMetabuild>;
  readonly metadata: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadata>;
  readonly name: Maybe<SchemasJsonDefinitionsPackagePropertiesName>;
  readonly namespaced_features: Maybe<SchemasJsonDefinitionsPackagePropertiesNamespaced_features>;
  readonly publish: Maybe<SchemasJsonDefinitionsPackagePropertiesPublish>;
  readonly publish_lockfile: Maybe<SchemasJsonDefinitionsPackagePropertiesPublish_lockfile>;
  readonly readme: Maybe<SchemasJsonDefinitionsPackagePropertiesReadme>;
  readonly repository: Maybe<SchemasJsonDefinitionsPackagePropertiesRepository>;
  readonly version: Maybe<SchemasJsonDefinitionsPackagePropertiesVersion>;
  readonly workspace: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspace>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthors = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAuthorsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenches = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenchesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobenchesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobins = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobinsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobinsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobinsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAutobinsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamples = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamplesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAutoexamplesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotests = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotestsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotestsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotestsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesAutotestsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesBuild = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesBuildFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategories = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesCategoriesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_run = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_runX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_runFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_runX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesDefault_runX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDescription = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDescriptionX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesDescriptionFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDescriptionX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesDescriptionX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentation = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentationX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentationFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentationX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesDocumentationX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesEdition = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesEditionFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExclude = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesExcludeX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesFilterInput = {
  readonly authors: Maybe<SchemasJsonDefinitionsPackagePropertiesAuthorsFilterInput>;
  readonly autobenches: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobenchesFilterInput>;
  readonly autobins: Maybe<SchemasJsonDefinitionsPackagePropertiesAutobinsFilterInput>;
  readonly autoexamples: Maybe<SchemasJsonDefinitionsPackagePropertiesAutoexamplesFilterInput>;
  readonly autotests: Maybe<SchemasJsonDefinitionsPackagePropertiesAutotestsFilterInput>;
  readonly build: Maybe<SchemasJsonDefinitionsPackagePropertiesBuildFilterInput>;
  readonly categories: Maybe<SchemasJsonDefinitionsPackagePropertiesCategoriesFilterInput>;
  readonly default_run: Maybe<SchemasJsonDefinitionsPackagePropertiesDefault_runFilterInput>;
  readonly description: Maybe<SchemasJsonDefinitionsPackagePropertiesDescriptionFilterInput>;
  readonly documentation: Maybe<SchemasJsonDefinitionsPackagePropertiesDocumentationFilterInput>;
  readonly edition: Maybe<SchemasJsonDefinitionsPackagePropertiesEditionFilterInput>;
  readonly exclude: Maybe<SchemasJsonDefinitionsPackagePropertiesExcludeFilterInput>;
  readonly homepage: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepageFilterInput>;
  readonly im_a_teapot: Maybe<SchemasJsonDefinitionsPackagePropertiesIm_a_teapotFilterInput>;
  readonly include: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeFilterInput>;
  readonly keywords: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsFilterInput>;
  readonly license: Maybe<SchemasJsonDefinitionsPackagePropertiesLicenseFilterInput>;
  readonly license_file: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_fileFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLinksFilterInput>;
  readonly metabuild: Maybe<SchemasJsonDefinitionsPackagePropertiesMetabuildFilterInput>;
  readonly metadata: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadataFilterInput>;
  readonly name: Maybe<SchemasJsonDefinitionsPackagePropertiesNameFilterInput>;
  readonly namespaced_features: Maybe<SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresFilterInput>;
  readonly publish: Maybe<SchemasJsonDefinitionsPackagePropertiesPublishFilterInput>;
  readonly publish_lockfile: Maybe<SchemasJsonDefinitionsPackagePropertiesPublish_lockfileFilterInput>;
  readonly readme: Maybe<SchemasJsonDefinitionsPackagePropertiesReadmeFilterInput>;
  readonly repository: Maybe<SchemasJsonDefinitionsPackagePropertiesRepositoryFilterInput>;
  readonly version: Maybe<SchemasJsonDefinitionsPackagePropertiesVersionFilterInput>;
  readonly workspace: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspaceFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepage = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepageX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepageFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepageX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepageX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepageX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepageX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesHomepageX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepageX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesHomepageX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIm_a_teapot = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIm_a_teapotX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesIm_a_teapotFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIm_a_teapotX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIm_a_teapotX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPackagePropertiesIm_a_teapotX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesInclude = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesIncludeX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesIncludeX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywords = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesKeywordsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLicenseX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_file = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_fileFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesLicense_fileX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicenseFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLicenseX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicenseX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLicenseX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesLicenseX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLicenseX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLicenseX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesLicenseX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLinks = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLinksX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesLinksFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesLinksX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLinksX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLinksX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesLinksX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesLinksX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesLinksX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesLinksX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesMetabuild = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesMetabuildX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesMetabuildFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesMetabuildX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesMetabuildX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPackagePropertiesMetabuildX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadata = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadataX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadataFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadataX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadataX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadataX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadataX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesMetadataX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadataX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesMetadataX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesName = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesNameX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesNameFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesNameX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesNamespaced_features = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPackagePropertiesNamespaced_featuresX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesNameX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesNameX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesNameX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesNameX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesNameX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesNameX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesPublish = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesPublish_lockfile = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesPublish_lockfileX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesPublish_lockfileFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesPublish_lockfileX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesPublish_lockfileX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPackagePropertiesPublish_lockfileX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesPublishFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesReadme = {
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesReadmeX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesReadmeFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesReadmeX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesReadmeX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesReadmeX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesReadmeX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesReadmeX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesReadmeX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesReadmeX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesRepository = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesRepositoryX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesRepositoryFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesRepositoryX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesRepositoryX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesVersion = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesVersionFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspace = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taplo>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspaceFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploLinks>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackagePropertiesWorkspaceX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPackageX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPackageX_taploLinks>;
};

type SchemasJsonDefinitionsPackageX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPackageX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPackageX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPackageX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPanic = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPanicX_taplo>;
};

type SchemasJsonDefinitionsPanicFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPanicX_taploFilterInput>;
};

type SchemasJsonDefinitionsPanicX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsPanicX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsPanicX_taploLinks>;
};

type SchemasJsonDefinitionsPanicX_taploDocs = {
  readonly enumValues: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonDefinitionsPanicX_taploDocsFilterInput = {
  readonly enumValues: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPanicX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsPanicX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsPanicX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPanicX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPanicX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatform = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPlatformProperties>;
};

type SchemasJsonDefinitionsPlatformFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPlatformPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPlatformProperties = {
  readonly build_dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependencies>;
  readonly dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependencies>;
  readonly dev_dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependencies>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesX_taplo>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependencies = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taplo>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploLinks>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPlatformPropertiesDependenciesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesX_taplo>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesX_taploFilterInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsPlatformPropertiesFilterInput = {
  readonly build_dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesBuild_dependenciesFilterInput>;
  readonly dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesDependenciesFilterInput>;
  readonly dev_dependencies: Maybe<SchemasJsonDefinitionsPlatformPropertiesDev_dependenciesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_author_pattern = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly pattern: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_author_patternFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly pattern: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_authors = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_authorsItems>;
};

type SchemasJsonDefinitionsPoetry_authorsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_authorsItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_authorsItems = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_authorsItemsFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_dependency = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_dependency_any = {
  readonly oneOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsPoetry_dependency_anyOneOf>>>;
};

type SchemasJsonDefinitionsPoetry_dependency_anyFilterInput = {
  readonly oneOf: Maybe<SchemasJsonDefinitionsPoetry_dependency_anyOneOfFilterListInput>;
};

type SchemasJsonDefinitionsPoetry_dependency_anyOneOf = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_dependency_anyOneOfFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_dependency_anyOneOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsPoetry_dependency_anyOneOfFilterInput>;
};

type SchemasJsonDefinitionsPoetry_dependencyFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_extra_script = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptProperties>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptProperties = {
  readonly callable: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesCallable>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtras>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesCallable = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesCallableFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_extra_scriptPropertiesFilterInput = {
  readonly callable: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesCallableFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_extra_scriptPropertiesExtrasFilterInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyProperties>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyProperties = {
  readonly file: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFile>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPython>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPlatform>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesMarkers>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesOptional>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtras>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFile = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFileFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFilterInput = {
  readonly file: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesFileFilterInput>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPythonFilterInput>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPlatformFilterInput>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesMarkersFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesOptionalFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_file_dependencyPropertiesExtrasFilterInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesMarkers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesMarkersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesOptional = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesOptionalFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPlatform = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPlatformFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_file_dependencyPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyProperties>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyProperties = {
  readonly git: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesGit>;
  readonly branch: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesBranch>;
  readonly tag: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesTag>;
  readonly rev: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesRev>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPython>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPlatform>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesMarkers>;
  readonly allow_prereleases: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllow_prereleases>;
  readonly allows_prereleases: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllows_prereleases>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesOptional>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtras>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllow_prereleases = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllow_prereleasesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllows_prereleases = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllows_prereleasesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesBranch = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesBranchFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesFilterInput = {
  readonly git: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesGitFilterInput>;
  readonly branch: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesBranchFilterInput>;
  readonly tag: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesTagFilterInput>;
  readonly rev: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesRevFilterInput>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPythonFilterInput>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPlatformFilterInput>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesMarkersFilterInput>;
  readonly allow_prereleases: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllow_prereleasesFilterInput>;
  readonly allows_prereleases: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesAllows_prereleasesFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesOptionalFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_git_dependencyPropertiesExtrasFilterInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesGit = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesGitFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesMarkers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesMarkersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesOptional = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesOptionalFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPlatform = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPlatformFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesRev = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesRevFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesTag = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_git_dependencyPropertiesTagFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyProperties>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyProperties = {
  readonly version: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesVersion>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPython>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPlatform>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesMarkers>;
  readonly allow_prereleases: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllow_prereleases>;
  readonly allows_prereleases: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllows_prereleases>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesOptional>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtras>;
  readonly source: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesSource>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllow_prereleases = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllow_prereleasesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllows_prereleases = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllows_prereleasesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesFilterInput = {
  readonly version: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesVersionFilterInput>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPythonFilterInput>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPlatformFilterInput>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesMarkersFilterInput>;
  readonly allow_prereleases: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllow_prereleasesFilterInput>;
  readonly allows_prereleases: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesAllows_prereleasesFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesOptionalFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesExtrasFilterInput>;
  readonly source: Maybe<SchemasJsonDefinitionsPoetry_long_dependencyPropertiesSourceFilterInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesMarkers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesMarkersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesOptional = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesOptionalFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPlatform = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPlatformFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesSource = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesSourceFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesVersion = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_long_dependencyPropertiesVersionFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_maintainers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_maintainersItems>;
};

type SchemasJsonDefinitionsPoetry_maintainersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_maintainersItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_maintainersItems = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_maintainersItemsFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly minItems: Maybe<Scalars['Int']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItems>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly minItems: Maybe<IntQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItems = {
  readonly oneOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOf>>>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsFilterInput = {
  readonly oneOf: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOfFilterListInput>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOf = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOfFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsPoetry_multiple_constraints_dependencyItemsOneOfFilterInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyProperties>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyProperties = {
  readonly path: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPath>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPython>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPlatform>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesMarkers>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesOptional>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtras>;
  readonly develop: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesDevelop>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesDevelop = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesDevelopFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesFilterInput = {
  readonly path: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPathFilterInput>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPythonFilterInput>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPlatformFilterInput>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesMarkersFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesOptionalFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesExtrasFilterInput>;
  readonly develop: Maybe<SchemasJsonDefinitionsPoetry_path_dependencyPropertiesDevelopFilterInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesMarkers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesMarkersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesOptional = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesOptionalFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPath = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPathFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPlatform = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPlatformFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_path_dependencyPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_pep440_version = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_pep440_versionFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_repository = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_repositoryProperties>;
};

type SchemasJsonDefinitionsPoetry_repositoryFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_repositoryProperties = {
  readonly name: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesName>;
  readonly url: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesUrl>;
  readonly default: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesDefault>;
  readonly secondary: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesSecondary>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesDefault = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesDefaultFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesFilterInput = {
  readonly name: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesNameFilterInput>;
  readonly url: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesUrlFilterInput>;
  readonly default: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesDefaultFilterInput>;
  readonly secondary: Maybe<SchemasJsonDefinitionsPoetry_repositoryPropertiesSecondaryFilterInput>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesName = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesNameFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesSecondary = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesSecondaryFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesUrl = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_repositoryPropertiesUrlFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_script = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_scriptFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_scripts = {
  readonly type: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties>;
};

type SchemasJsonDefinitionsPoetry_scriptsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly oneOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOf>>>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly oneOf: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOfFilterListInput>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOf = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOfFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxOneOfFilterInput>;
};

type SchemasJsonDefinitionsPoetry_scriptsPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonDefinitionsPoetry_scriptsPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependency = {
  readonly type: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyProperties>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesFilterInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyProperties = {
  readonly url: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesUrl>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPython>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPlatform>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesMarkers>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesOptional>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtras>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasItems>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasItemsFilterInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesFilterInput = {
  readonly url: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesUrlFilterInput>;
  readonly python: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPythonFilterInput>;
  readonly platform: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPlatformFilterInput>;
  readonly markers: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesMarkersFilterInput>;
  readonly optional: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesOptionalFilterInput>;
  readonly extras: Maybe<SchemasJsonDefinitionsPoetry_url_dependencyPropertiesExtrasFilterInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesMarkers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesMarkersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesOptional = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesOptionalFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPlatform = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPlatformFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesUrl = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPoetry_url_dependencyPropertiesUrlFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfile = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsProfileProperties>;
};

type SchemasJsonDefinitionsProfileFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsProfilePropertiesFilterInput>;
};

type SchemasJsonDefinitionsProfileProperties = {
  readonly build_override: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_override>;
  readonly codegen_units: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_units>;
  readonly debug: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug>;
  readonly debug_assertions: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertions>;
  readonly dir_name: Maybe<SchemasJsonDefinitionsProfilePropertiesDir_name>;
  readonly incremental: Maybe<SchemasJsonDefinitionsProfilePropertiesIncremental>;
  readonly inherits: Maybe<SchemasJsonDefinitionsProfilePropertiesInherits>;
  readonly lto: Maybe<SchemasJsonDefinitionsProfilePropertiesLto>;
  readonly opt_level: Maybe<SchemasJsonDefinitionsProfilePropertiesOpt_level>;
  readonly overflow_checks: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checks>;
  readonly package: Maybe<SchemasJsonDefinitionsProfilePropertiesPackage>;
  readonly panic: Maybe<SchemasJsonDefinitionsProfilePropertiesPanic>;
  readonly rpath: Maybe<SchemasJsonDefinitionsProfilePropertiesRpath>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_override = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesBuild_overrideX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_units = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
  readonly minimum: Maybe<Scalars['Int']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_unitsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
  readonly minimum: Maybe<IntQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesCodegen_unitsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertions = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertionsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesDebug_assertionsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDebugFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDir_name = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesDir_nameX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesDir_nameFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesDir_nameX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesDir_nameX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsProfilePropertiesDir_nameX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesFilterInput = {
  readonly build_override: Maybe<SchemasJsonDefinitionsProfilePropertiesBuild_overrideFilterInput>;
  readonly codegen_units: Maybe<SchemasJsonDefinitionsProfilePropertiesCodegen_unitsFilterInput>;
  readonly debug: Maybe<SchemasJsonDefinitionsProfilePropertiesDebugFilterInput>;
  readonly debug_assertions: Maybe<SchemasJsonDefinitionsProfilePropertiesDebug_assertionsFilterInput>;
  readonly dir_name: Maybe<SchemasJsonDefinitionsProfilePropertiesDir_nameFilterInput>;
  readonly incremental: Maybe<SchemasJsonDefinitionsProfilePropertiesIncrementalFilterInput>;
  readonly inherits: Maybe<SchemasJsonDefinitionsProfilePropertiesInheritsFilterInput>;
  readonly lto: Maybe<SchemasJsonDefinitionsProfilePropertiesLtoFilterInput>;
  readonly opt_level: Maybe<SchemasJsonDefinitionsProfilePropertiesOpt_levelFilterInput>;
  readonly overflow_checks: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checksFilterInput>;
  readonly package: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageFilterInput>;
  readonly panic: Maybe<SchemasJsonDefinitionsProfilePropertiesPanicFilterInput>;
  readonly rpath: Maybe<SchemasJsonDefinitionsProfilePropertiesRpathFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesIncremental = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesIncrementalX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesIncrementalFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesIncrementalX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesIncrementalX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesInherits = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesInheritsX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesInheritsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesInheritsX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesInheritsX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsProfilePropertiesInheritsX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesLto = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesLtoFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesOpt_level = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesOpt_levelFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checks = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checksFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesOverflow_checksX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesPackage = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesPackageX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesPackageX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesPanic = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesPanicFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilePropertiesRpath = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesRpathX_taplo>;
};

type SchemasJsonDefinitionsProfilePropertiesRpathFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilePropertiesRpathX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesRpathX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesRpathX_taploLinks>;
};

type SchemasJsonDefinitionsProfilePropertiesRpathX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilePropertiesRpathX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilePropertiesRpathX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilePropertiesRpathX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfiles = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsProfilesProperties>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsProfilesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesX_taplo>;
};

type SchemasJsonDefinitionsProfilesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsProfilesPropertiesFilterInput>;
  readonly additionalProperties: Maybe<SchemasJsonDefinitionsProfilesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilesProperties = {
  readonly bench: Maybe<SchemasJsonDefinitionsProfilesPropertiesBench>;
  readonly dev: Maybe<SchemasJsonDefinitionsProfilesPropertiesDev>;
  readonly release: Maybe<SchemasJsonDefinitionsProfilesPropertiesRelease>;
  readonly test: Maybe<SchemasJsonDefinitionsProfilesPropertiesTest>;
};

type SchemasJsonDefinitionsProfilesPropertiesBench = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taplo>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taploLinks>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesBenchX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesDev = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taplo>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taploLinks>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesDevX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesFilterInput = {
  readonly bench: Maybe<SchemasJsonDefinitionsProfilesPropertiesBenchFilterInput>;
  readonly dev: Maybe<SchemasJsonDefinitionsProfilesPropertiesDevFilterInput>;
  readonly release: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseFilterInput>;
  readonly test: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesRelease = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taplo>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploLinks>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesReleaseX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesTest = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taplo>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taploFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taplo = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taploDocs>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taploLinks>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonDefinitionsProfilesPropertiesTestX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesPropertiesTestX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsProfilesX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilesX_taploLinks>;
};

type SchemasJsonDefinitionsProfilesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsProfilesX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsProfilesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsProfilesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPublish = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsPublishAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPublishX_taplo>;
};

type SchemasJsonDefinitionsPublishAnyOf = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsPublishAnyOfItems>;
};

type SchemasJsonDefinitionsPublishAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsPublishAnyOfItemsFilterInput>;
};

type SchemasJsonDefinitionsPublishAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsPublishAnyOfFilterInput>;
};

type SchemasJsonDefinitionsPublishAnyOfItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPublishAnyOfItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsPublishFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsPublishAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsPublishX_taploFilterInput>;
};

type SchemasJsonDefinitionsPublishX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsPublishX_taploLinks>;
};

type SchemasJsonDefinitionsPublishX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsPublishX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsPublishX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsPublishX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsReadme = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly anyOf: Maybe<ReadonlyArray<Maybe<SchemasJsonDefinitionsReadmeAnyOf>>>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsReadmeX_taplo>;
};

type SchemasJsonDefinitionsReadmeAnyOf = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsReadmeAnyOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsReadmeAnyOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonDefinitionsReadmeAnyOfFilterInput>;
};

type SchemasJsonDefinitionsReadmeFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly anyOf: Maybe<SchemasJsonDefinitionsReadmeAnyOfFilterListInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsReadmeX_taploFilterInput>;
};

type SchemasJsonDefinitionsReadmeX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsReadmeX_taploLinks>;
};

type SchemasJsonDefinitionsReadmeX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsReadmeX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsReadmeX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsReadmeX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsSemVer = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly pattern: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsSemVerX_taplo>;
};

type SchemasJsonDefinitionsSemVerFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly pattern: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsSemVerX_taploFilterInput>;
};

type SchemasJsonDefinitionsSemVerRequirement = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsSemVerRequirementX_taplo>;
};

type SchemasJsonDefinitionsSemVerRequirementFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsSemVerRequirementX_taploFilterInput>;
};

type SchemasJsonDefinitionsSemVerRequirementX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsSemVerRequirementX_taploLinks>;
};

type SchemasJsonDefinitionsSemVerRequirementX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsSemVerRequirementX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsSemVerRequirementX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsSemVerRequirementX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsSemVerX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsSemVerX_taploLinks>;
};

type SchemasJsonDefinitionsSemVerX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsSemVerX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsSemVerX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsSemVerX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTarget = {
  readonly title: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsTargetProperties>;
};

type SchemasJsonDefinitionsTargetFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsTargetPropertiesFilterInput>;
};

type SchemasJsonDefinitionsTargetProperties = {
  readonly bench: Maybe<SchemasJsonDefinitionsTargetPropertiesBench>;
  readonly crate_type: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_type>;
  readonly doc: Maybe<SchemasJsonDefinitionsTargetPropertiesDoc>;
  readonly doctest: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctest>;
  readonly edition: Maybe<SchemasJsonDefinitionsTargetPropertiesEdition>;
  readonly harness: Maybe<SchemasJsonDefinitionsTargetPropertiesHarness>;
  readonly name: Maybe<SchemasJsonDefinitionsTargetPropertiesName>;
  readonly path: Maybe<SchemasJsonDefinitionsTargetPropertiesPath>;
  readonly plugin: Maybe<SchemasJsonDefinitionsTargetPropertiesPlugin>;
  readonly proc_macro: Maybe<SchemasJsonDefinitionsTargetPropertiesProc_macro>;
  readonly required_features: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_features>;
  readonly test: Maybe<SchemasJsonDefinitionsTargetPropertiesTest>;
};

type SchemasJsonDefinitionsTargetPropertiesBench = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesBenchX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesBenchFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesBenchX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesBenchX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesBenchX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesBenchX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesBenchX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesBenchX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesBenchX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_type = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeItems = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeItemsX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsTargetPropertiesCrate_typeX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDoc = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesDocX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesDocFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesDocX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctest = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctestX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctestFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctestX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctestX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctestX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctestX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctestX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctestX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesDoctestX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDocX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesDocX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesDocX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesDocX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesDocX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesDocX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesEdition = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesEditionFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesFilterInput = {
  readonly bench: Maybe<SchemasJsonDefinitionsTargetPropertiesBenchFilterInput>;
  readonly crate_type: Maybe<SchemasJsonDefinitionsTargetPropertiesCrate_typeFilterInput>;
  readonly doc: Maybe<SchemasJsonDefinitionsTargetPropertiesDocFilterInput>;
  readonly doctest: Maybe<SchemasJsonDefinitionsTargetPropertiesDoctestFilterInput>;
  readonly edition: Maybe<SchemasJsonDefinitionsTargetPropertiesEditionFilterInput>;
  readonly harness: Maybe<SchemasJsonDefinitionsTargetPropertiesHarnessFilterInput>;
  readonly name: Maybe<SchemasJsonDefinitionsTargetPropertiesNameFilterInput>;
  readonly path: Maybe<SchemasJsonDefinitionsTargetPropertiesPathFilterInput>;
  readonly plugin: Maybe<SchemasJsonDefinitionsTargetPropertiesPluginFilterInput>;
  readonly proc_macro: Maybe<SchemasJsonDefinitionsTargetPropertiesProc_macroFilterInput>;
  readonly required_features: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresFilterInput>;
  readonly test: Maybe<SchemasJsonDefinitionsTargetPropertiesTestFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesHarness = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesHarnessX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesHarnessFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesHarnessX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesHarnessX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesHarnessX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesHarnessX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesHarnessX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesHarnessX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesHarnessX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesName = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesNameX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesNameFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesNameX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesNameX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesNameX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesNameX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesNameX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesNameX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesNameX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesPath = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesPathX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesPathFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesPathX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesPathX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesPathX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesPathX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesPathX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesPathX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesPathX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesPlugin = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesPluginX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesPluginFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesPluginX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesPluginX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsTargetPropertiesPluginX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesProc_macro = {
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesProc_macroX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesProc_macroFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesProc_macroX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesProc_macroX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonDefinitionsTargetPropertiesProc_macroX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_features = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesRequired_featuresX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsTargetPropertiesTest = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesTestX_taplo>;
};

type SchemasJsonDefinitionsTargetPropertiesTestFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsTargetPropertiesTestX_taploFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesTestX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesTestX_taploLinks>;
};

type SchemasJsonDefinitionsTargetPropertiesTestX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsTargetPropertiesTestX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsTargetPropertiesTestX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsTargetPropertiesTestX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspace = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly properties: Maybe<SchemasJsonDefinitionsWorkspaceProperties>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspaceX_taplo>;
};

type SchemasJsonDefinitionsWorkspaceFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonDefinitionsWorkspacePropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspaceX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspaceProperties = {
  readonly default_members: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_members>;
  readonly exclude: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExclude>;
  readonly members: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembers>;
  readonly metadata: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadata>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_members = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesDefault_membersX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExclude = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesExcludeX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesFilterInput = {
  readonly default_members: Maybe<SchemasJsonDefinitionsWorkspacePropertiesDefault_membersFilterInput>;
  readonly exclude: Maybe<SchemasJsonDefinitionsWorkspacePropertiesExcludeFilterInput>;
  readonly members: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersFilterInput>;
  readonly metadata: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadataFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembers = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItems>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMembersX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadata = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taplo>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadataFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspacePropertiesMetadataX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonDefinitionsWorkspaceX_taplo = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspaceX_taploLinks>;
};

type SchemasJsonDefinitionsWorkspaceX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonDefinitionsWorkspaceX_taploLinksFilterInput>;
};

type SchemasJsonDefinitionsWorkspaceX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonDefinitionsWorkspaceX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonEdge = {
  readonly next: Maybe<SchemasJson>;
  readonly node: SchemasJson;
  readonly previous: Maybe<SchemasJson>;
};

enum SchemasJsonFieldsEnum {
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type',
  _schema = '_schema',
  title = 'title',
  type = 'type',
  description = 'description',
  x_taplo_info___authors = 'x_taplo_info.authors',
  x_taplo_info___patterns = 'x_taplo_info.patterns',
  properties___tab_spaces___type = 'properties.tab_spaces.type',
  properties___tab_spaces___description = 'properties.tab_spaces.description',
  properties___tab_spaces___default = 'properties.tab_spaces.default',
  properties___fn_args_layout___type = 'properties.fn_args_layout.type',
  properties___fn_args_layout___description = 'properties.fn_args_layout.description',
  properties___fn_args_layout___default = 'properties.fn_args_layout.default',
  properties___fn_args_layout___enum = 'properties.fn_args_layout.enum',
  properties___merge_derives___type = 'properties.merge_derives.type',
  properties___merge_derives___description = 'properties.merge_derives.description',
  properties___merge_derives___default = 'properties.merge_derives.default',
  properties___merge_derives___enum = 'properties.merge_derives.enum',
  properties___print_misformatted_file_names___type = 'properties.print_misformatted_file_names.type',
  properties___print_misformatted_file_names___description = 'properties.print_misformatted_file_names.description',
  properties___print_misformatted_file_names___default = 'properties.print_misformatted_file_names.default',
  properties___print_misformatted_file_names___enum = 'properties.print_misformatted_file_names.enum',
  properties___remove_nested_parens___type = 'properties.remove_nested_parens.type',
  properties___remove_nested_parens___description = 'properties.remove_nested_parens.description',
  properties___remove_nested_parens___default = 'properties.remove_nested_parens.default',
  properties___remove_nested_parens___enum = 'properties.remove_nested_parens.enum',
  properties___use_small_heuristics___type = 'properties.use_small_heuristics.type',
  properties___use_small_heuristics___description = 'properties.use_small_heuristics.description',
  properties___use_small_heuristics___default = 'properties.use_small_heuristics.default',
  properties___use_small_heuristics___enum = 'properties.use_small_heuristics.enum',
  properties___use_try_shorthand___type = 'properties.use_try_shorthand.type',
  properties___use_try_shorthand___description = 'properties.use_try_shorthand.description',
  properties___use_try_shorthand___default = 'properties.use_try_shorthand.default',
  properties___use_try_shorthand___enum = 'properties.use_try_shorthand.enum',
  properties___reorder_modules___type = 'properties.reorder_modules.type',
  properties___reorder_modules___description = 'properties.reorder_modules.description',
  properties___reorder_modules___default = 'properties.reorder_modules.default',
  properties___reorder_modules___enum = 'properties.reorder_modules.enum',
  properties___hard_tabs___type = 'properties.hard_tabs.type',
  properties___hard_tabs___description = 'properties.hard_tabs.description',
  properties___hard_tabs___default = 'properties.hard_tabs.default',
  properties___hard_tabs___enum = 'properties.hard_tabs.enum',
  properties___use_field_init_shorthand___type = 'properties.use_field_init_shorthand.type',
  properties___use_field_init_shorthand___description = 'properties.use_field_init_shorthand.description',
  properties___use_field_init_shorthand___default = 'properties.use_field_init_shorthand.default',
  properties___use_field_init_shorthand___enum = 'properties.use_field_init_shorthand.enum',
  properties___max_width___type = 'properties.max_width.type',
  properties___max_width___description = 'properties.max_width.description',
  properties___max_width___default = 'properties.max_width.default',
  properties___reorder_imports___type = 'properties.reorder_imports.type',
  properties___reorder_imports___description = 'properties.reorder_imports.description',
  properties___reorder_imports___default = 'properties.reorder_imports.default',
  properties___reorder_imports___enum = 'properties.reorder_imports.enum',
  properties___match_arm_leading_pipes___type = 'properties.match_arm_leading_pipes.type',
  properties___match_arm_leading_pipes___description = 'properties.match_arm_leading_pipes.description',
  properties___match_arm_leading_pipes___default = 'properties.match_arm_leading_pipes.default',
  properties___match_arm_leading_pipes___enum = 'properties.match_arm_leading_pipes.enum',
  properties___force_explicit_abi___type = 'properties.force_explicit_abi.type',
  properties___force_explicit_abi___description = 'properties.force_explicit_abi.description',
  properties___force_explicit_abi___default = 'properties.force_explicit_abi.default',
  properties___force_explicit_abi___enum = 'properties.force_explicit_abi.enum',
  properties___edition___type = 'properties.edition.type',
  properties___edition___description = 'properties.edition.description',
  properties___edition___default = 'properties.edition.default',
  properties___edition___enum = 'properties.edition.enum',
  properties___newline_style___type = 'properties.newline_style.type',
  properties___newline_style___description = 'properties.newline_style.description',
  properties___newline_style___default = 'properties.newline_style.default',
  properties___newline_style___enum = 'properties.newline_style.enum',
  properties___build_system____ref = 'properties.build_system._ref',
  properties___tool___type = 'properties.tool.type',
  properties___tool___description = 'properties.tool.description',
  properties___tool___additionalProperties = 'properties.tool.additionalProperties',
  properties___badges___description = 'properties.badges.description',
  properties___badges___type = 'properties.badges.type',
  properties___badges___additionalProperties___type = 'properties.badges.additionalProperties.type',
  properties___bench___description = 'properties.bench.description',
  properties___bench___type = 'properties.bench.type',
  properties___bench___items___description = 'properties.bench.items.description',
  properties___bench___items____ref = 'properties.bench.items._ref',
  properties___bin___description = 'properties.bin.description',
  properties___bin___type = 'properties.bin.type',
  properties___bin___items___description = 'properties.bin.items.description',
  properties___bin___items____ref = 'properties.bin.items._ref',
  properties___build_dependencies___type = 'properties.build_dependencies.type',
  properties___build_dependencies___additionalProperties____ref = 'properties.build_dependencies.additionalProperties._ref',
  properties___build_dependencies___x_taplo___hidden = 'properties.build_dependencies.x_taplo.hidden',
  properties___cargo_features___type = 'properties.cargo_features.type',
  properties___cargo_features___items___type = 'properties.cargo_features.items.type',
  properties___dependencies___description = 'properties.dependencies.description',
  properties___dependencies___type = 'properties.dependencies.type',
  properties___dependencies___additionalProperties____ref = 'properties.dependencies.additionalProperties._ref',
  properties___dev_dependencies___type = 'properties.dev_dependencies.type',
  properties___dev_dependencies___additionalProperties____ref = 'properties.dev_dependencies.additionalProperties._ref',
  properties___dev_dependencies___x_taplo___hidden = 'properties.dev_dependencies.x_taplo.hidden',
  properties___example___description = 'properties.example.description',
  properties___example___type = 'properties.example.type',
  properties___example___items___description = 'properties.example.items.description',
  properties___example___items____ref = 'properties.example.items._ref',
  properties___features___description = 'properties.features.description',
  properties___features___type = 'properties.features.type',
  properties___features___additionalProperties___type = 'properties.features.additionalProperties.type',
  properties___lib____ref = 'properties.lib._ref',
  properties___package____ref = 'properties.package._ref',
  properties___patch___description = 'properties.patch.description',
  properties___patch___type = 'properties.patch.type',
  properties___patch___additionalProperties___type = 'properties.patch.additionalProperties.type',
  properties___profile____ref = 'properties.profile._ref',
  properties___project____ref = 'properties.project._ref',
  properties___project___x_taplo___hidden = 'properties.project.x_taplo.hidden',
  properties___replace___type = 'properties.replace.type',
  properties___replace___additionalProperties____ref = 'properties.replace.additionalProperties._ref',
  properties___replace___x_taplo___hidden = 'properties.replace.x_taplo.hidden',
  properties___target___type = 'properties.target.type',
  properties___target___additionalProperties____ref = 'properties.target.additionalProperties._ref',
  properties___test___description = 'properties.test.description',
  properties___test___type = 'properties.test.type',
  properties___test___items___description = 'properties.test.items.description',
  properties___test___items____ref = 'properties.test.items._ref',
  properties___workspace____ref = 'properties.workspace._ref',
  definitions___poetry_author_pattern___description = 'definitions.poetry_author_pattern.description',
  definitions___poetry_author_pattern___type = 'definitions.poetry_author_pattern.type',
  definitions___poetry_author_pattern___pattern = 'definitions.poetry_author_pattern.pattern',
  definitions___poetry_authors___type = 'definitions.poetry_authors.type',
  definitions___poetry_authors___description = 'definitions.poetry_authors.description',
  definitions___poetry_authors___items____ref = 'definitions.poetry_authors.items._ref',
  definitions___poetry_maintainers___type = 'definitions.poetry_maintainers.type',
  definitions___poetry_maintainers___description = 'definitions.poetry_maintainers.description',
  definitions___poetry_maintainers___items____ref = 'definitions.poetry_maintainers.items._ref',
  definitions___poetry_dependency_any___oneOf = 'definitions.poetry_dependency_any.oneOf',
  definitions___poetry_dependency_any___oneOf____ref = 'definitions.poetry_dependency_any.oneOf._ref',
  definitions___poetry_pep440_version___type = 'definitions.poetry_pep440_version.type',
  definitions___poetry_pep440_version___description = 'definitions.poetry_pep440_version.description',
  definitions___poetry_dependency____ref = 'definitions.poetry_dependency._ref',
  definitions___poetry_long_dependency___type = 'definitions.poetry_long_dependency.type',
  definitions___poetry_long_dependency___required = 'definitions.poetry_long_dependency.required',
  definitions___poetry_long_dependency___additionalProperties = 'definitions.poetry_long_dependency.additionalProperties',
  definitions___poetry_git_dependency___type = 'definitions.poetry_git_dependency.type',
  definitions___poetry_git_dependency___required = 'definitions.poetry_git_dependency.required',
  definitions___poetry_git_dependency___additionalProperties = 'definitions.poetry_git_dependency.additionalProperties',
  definitions___poetry_file_dependency___type = 'definitions.poetry_file_dependency.type',
  definitions___poetry_file_dependency___required = 'definitions.poetry_file_dependency.required',
  definitions___poetry_file_dependency___additionalProperties = 'definitions.poetry_file_dependency.additionalProperties',
  definitions___poetry_path_dependency___type = 'definitions.poetry_path_dependency.type',
  definitions___poetry_path_dependency___required = 'definitions.poetry_path_dependency.required',
  definitions___poetry_path_dependency___additionalProperties = 'definitions.poetry_path_dependency.additionalProperties',
  definitions___poetry_url_dependency___type = 'definitions.poetry_url_dependency.type',
  definitions___poetry_url_dependency___required = 'definitions.poetry_url_dependency.required',
  definitions___poetry_url_dependency___additionalProperties = 'definitions.poetry_url_dependency.additionalProperties',
  definitions___poetry_multiple_constraints_dependency___type = 'definitions.poetry_multiple_constraints_dependency.type',
  definitions___poetry_multiple_constraints_dependency___minItems = 'definitions.poetry_multiple_constraints_dependency.minItems',
  definitions___poetry_multiple_constraints_dependency___items___oneOf = 'definitions.poetry_multiple_constraints_dependency.items.oneOf',
  definitions___poetry_scripts___type = 'definitions.poetry_scripts.type',
  definitions___poetry_script___type = 'definitions.poetry_script.type',
  definitions___poetry_script___description = 'definitions.poetry_script.description',
  definitions___poetry_extra_script___type = 'definitions.poetry_extra_script.type',
  definitions___poetry_extra_script___description = 'definitions.poetry_extra_script.description',
  definitions___poetry_extra_script___additionalProperties = 'definitions.poetry_extra_script.additionalProperties',
  definitions___poetry_repository___type = 'definitions.poetry_repository.type',
  definitions___poetry_repository___additionalProperties = 'definitions.poetry_repository.additionalProperties',
  definitions___BuildSystem___title = 'definitions.BuildSystem.title',
  definitions___BuildSystem___type = 'definitions.BuildSystem.type',
  definitions___BuildSystem___description = 'definitions.BuildSystem.description',
  definitions___BuildSystem___required = 'definitions.BuildSystem.required',
  definitions___Build___title = 'definitions.Build.title',
  definitions___Build___description = 'definitions.Build.description',
  definitions___Build___anyOf = 'definitions.Build.anyOf',
  definitions___Build___anyOf___type = 'definitions.Build.anyOf.type',
  definitions___DebugLevel___title = 'definitions.DebugLevel.title',
  definitions___DebugLevel___description = 'definitions.DebugLevel.description',
  definitions___DebugLevel___anyOf = 'definitions.DebugLevel.anyOf',
  definitions___DebugLevel___anyOf___type = 'definitions.DebugLevel.anyOf.type',
  definitions___DebugLevel___anyOf___format = 'definitions.DebugLevel.anyOf.format',
  definitions___DebugLevel___anyOf___minimum = 'definitions.DebugLevel.anyOf.minimum',
  definitions___Dependency___title = 'definitions.Dependency.title',
  definitions___Dependency___anyOf = 'definitions.Dependency.anyOf',
  definitions___Dependency___anyOf____ref = 'definitions.Dependency.anyOf._ref',
  definitions___DetailedDependency___title = 'definitions.DetailedDependency.title',
  definitions___DetailedDependency___type = 'definitions.DetailedDependency.type',
  definitions___DetailedDependency___x_taplo___initFields = 'definitions.DetailedDependency.x_taplo.initFields',
  definitions___Edition___title = 'definitions.Edition.title',
  definitions___Edition___description = 'definitions.Edition.description',
  definitions___Edition___type = 'definitions.Edition.type',
  definitions___Edition___enum = 'definitions.Edition.enum',
  definitions___Lto___title = 'definitions.Lto.title',
  definitions___Lto___description = 'definitions.Lto.description',
  definitions___Lto___anyOf = 'definitions.Lto.anyOf',
  definitions___Lto___anyOf___type = 'definitions.Lto.anyOf.type',
  definitions___MetaBuild___title = 'definitions.MetaBuild.title',
  definitions___MetaBuild___type = 'definitions.MetaBuild.type',
  definitions___MetaBuild___items___type = 'definitions.MetaBuild.items.type',
  definitions___OptLevel___title = 'definitions.OptLevel.title',
  definitions___OptLevel___description = 'definitions.OptLevel.description',
  definitions___OptLevel___anyOf = 'definitions.OptLevel.anyOf',
  definitions___OptLevel___anyOf___type = 'definitions.OptLevel.anyOf.type',
  definitions___OptLevel___anyOf___format = 'definitions.OptLevel.anyOf.format',
  definitions___Package___title = 'definitions.Package.title',
  definitions___Package___description = 'definitions.Package.description',
  definitions___Package___type = 'definitions.Package.type',
  definitions___Package___required = 'definitions.Package.required',
  definitions___Panic___title = 'definitions.Panic.title',
  definitions___Panic___description = 'definitions.Panic.description',
  definitions___Panic___type = 'definitions.Panic.type',
  definitions___Panic___enum = 'definitions.Panic.enum',
  definitions___Platform___title = 'definitions.Platform.title',
  definitions___Platform___type = 'definitions.Platform.type',
  definitions___Profile___title = 'definitions.Profile.title',
  definitions___Profile___type = 'definitions.Profile.type',
  definitions___Profiles___title = 'definitions.Profiles.title',
  definitions___Profiles___description = 'definitions.Profiles.description',
  definitions___Profiles___type = 'definitions.Profiles.type',
  definitions___Profiles___additionalProperties____ref = 'definitions.Profiles.additionalProperties._ref',
  definitions___Publish___title = 'definitions.Publish.title',
  definitions___Publish___description = 'definitions.Publish.description',
  definitions___Publish___anyOf = 'definitions.Publish.anyOf',
  definitions___Publish___anyOf___type = 'definitions.Publish.anyOf.type',
  definitions___Readme___title = 'definitions.Readme.title',
  definitions___Readme___description = 'definitions.Readme.description',
  definitions___Readme___anyOf = 'definitions.Readme.anyOf',
  definitions___Readme___anyOf___type = 'definitions.Readme.anyOf.type',
  definitions___SemVer___title = 'definitions.SemVer.title',
  definitions___SemVer___description = 'definitions.SemVer.description',
  definitions___SemVer___default = 'definitions.SemVer.default',
  definitions___SemVer___type = 'definitions.SemVer.type',
  definitions___SemVer___pattern = 'definitions.SemVer.pattern',
  definitions___SemVerRequirement___title = 'definitions.SemVerRequirement.title',
  definitions___SemVerRequirement___description = 'definitions.SemVerRequirement.description',
  definitions___SemVerRequirement___default = 'definitions.SemVerRequirement.default',
  definitions___SemVerRequirement___type = 'definitions.SemVerRequirement.type',
  definitions___Target___title = 'definitions.Target.title',
  definitions___Target___type = 'definitions.Target.type',
  definitions___Workspace___title = 'definitions.Workspace.title',
  definitions___Workspace___description = 'definitions.Workspace.description',
  definitions___Workspace___type = 'definitions.Workspace.type',
  additionalProperties = 'additionalProperties'
}

type SchemasJsonFilterInput = {
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
  readonly _schema: Maybe<StringQueryOperatorInput>;
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly x_taplo_info: Maybe<SchemasJsonX_taplo_infoFilterInput>;
  readonly properties: Maybe<SchemasJsonPropertiesFilterInput>;
  readonly definitions: Maybe<SchemasJsonDefinitionsFilterInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SchemasJsonEdge>;
  readonly nodes: ReadonlyArray<SchemasJson>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type SchemasJsonProperties = {
  readonly tab_spaces: Maybe<SchemasJsonPropertiesTab_spaces>;
  readonly fn_args_layout: Maybe<SchemasJsonPropertiesFn_args_layout>;
  readonly merge_derives: Maybe<SchemasJsonPropertiesMerge_derives>;
  readonly print_misformatted_file_names: Maybe<SchemasJsonPropertiesPrint_misformatted_file_names>;
  readonly remove_nested_parens: Maybe<SchemasJsonPropertiesRemove_nested_parens>;
  readonly use_small_heuristics: Maybe<SchemasJsonPropertiesUse_small_heuristics>;
  readonly use_try_shorthand: Maybe<SchemasJsonPropertiesUse_try_shorthand>;
  readonly reorder_modules: Maybe<SchemasJsonPropertiesReorder_modules>;
  readonly hard_tabs: Maybe<SchemasJsonPropertiesHard_tabs>;
  readonly use_field_init_shorthand: Maybe<SchemasJsonPropertiesUse_field_init_shorthand>;
  readonly max_width: Maybe<SchemasJsonPropertiesMax_width>;
  readonly reorder_imports: Maybe<SchemasJsonPropertiesReorder_imports>;
  readonly match_arm_leading_pipes: Maybe<SchemasJsonPropertiesMatch_arm_leading_pipes>;
  readonly force_explicit_abi: Maybe<SchemasJsonPropertiesForce_explicit_abi>;
  readonly edition: Maybe<SchemasJsonPropertiesEdition>;
  readonly newline_style: Maybe<SchemasJsonPropertiesNewline_style>;
  readonly build_system: Maybe<SchemasJsonPropertiesBuild_system>;
  readonly tool: Maybe<SchemasJsonPropertiesTool>;
  readonly badges: Maybe<SchemasJsonPropertiesBadges>;
  readonly bench: Maybe<SchemasJsonPropertiesBench>;
  readonly bin: Maybe<SchemasJsonPropertiesBin>;
  readonly build_dependencies: Maybe<SchemasJsonPropertiesBuild_dependencies>;
  readonly cargo_features: Maybe<SchemasJsonPropertiesCargo_features>;
  readonly dependencies: Maybe<SchemasJsonPropertiesDependencies>;
  readonly dev_dependencies: Maybe<SchemasJsonPropertiesDev_dependencies>;
  readonly example: Maybe<SchemasJsonPropertiesExample>;
  readonly features: Maybe<SchemasJsonPropertiesFeatures>;
  readonly lib: Maybe<SchemasJsonPropertiesLib>;
  readonly package: Maybe<SchemasJsonPropertiesPackage>;
  readonly patch: Maybe<SchemasJsonPropertiesPatch>;
  readonly profile: Maybe<SchemasJsonPropertiesProfile>;
  readonly project: Maybe<SchemasJsonPropertiesProject>;
  readonly replace: Maybe<SchemasJsonPropertiesReplace>;
  readonly target: Maybe<SchemasJsonPropertiesTarget>;
  readonly test: Maybe<SchemasJsonPropertiesTest>;
  readonly workspace: Maybe<SchemasJsonPropertiesWorkspace>;
};

type SchemasJsonPropertiesBadges = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBadgesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBadgesX_taplo>;
};

type SchemasJsonPropertiesBadgesAdditionalProperties = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBadgesAdditionalPropertiesAdditionalProperties>;
};

type SchemasJsonPropertiesBadgesAdditionalPropertiesAdditionalProperties = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBadgesAdditionalPropertiesAdditionalPropertiesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBadgesAdditionalPropertiesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBadgesAdditionalPropertiesAdditionalPropertiesFilterInput>;
};

type SchemasJsonPropertiesBadgesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBadgesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBadgesX_taploFilterInput>;
};

type SchemasJsonPropertiesBadgesX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesBadgesX_taploLinks>;
};

type SchemasJsonPropertiesBadgesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesBadgesX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesBadgesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBadgesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBench = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesBenchItems>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBenchX_taplo>;
};

type SchemasJsonPropertiesBenchFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesBenchItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBenchX_taploFilterInput>;
};

type SchemasJsonPropertiesBenchItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBenchItemsX_taplo>;
};

type SchemasJsonPropertiesBenchItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBenchItemsX_taploFilterInput>;
};

type SchemasJsonPropertiesBenchItemsX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesBenchItemsX_taploLinks>;
};

type SchemasJsonPropertiesBenchItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesBenchItemsX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesBenchItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBenchItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBenchX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesBenchX_taploLinks>;
};

type SchemasJsonPropertiesBenchX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesBenchX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesBenchX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBenchX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBin = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesBinItems>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBinX_taplo>;
};

type SchemasJsonPropertiesBinFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesBinItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBinX_taploFilterInput>;
};

type SchemasJsonPropertiesBinItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBinItemsX_taplo>;
};

type SchemasJsonPropertiesBinItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBinItemsX_taploFilterInput>;
};

type SchemasJsonPropertiesBinItemsX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesBinItemsX_taploLinks>;
};

type SchemasJsonPropertiesBinItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesBinItemsX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesBinItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBinItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBinX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesBinX_taploLinks>;
};

type SchemasJsonPropertiesBinX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesBinX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesBinX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBinX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBuild_dependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBuild_dependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBuild_dependenciesX_taplo>;
};

type SchemasJsonPropertiesBuild_dependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBuild_dependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesBuild_dependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesBuild_dependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesBuild_dependenciesX_taploFilterInput>;
};

type SchemasJsonPropertiesBuild_dependenciesX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonPropertiesBuild_dependenciesX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesBuild_system = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesBuild_systemFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesCargo_features = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesCargo_featuresItems>;
};

type SchemasJsonPropertiesCargo_featuresFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesCargo_featuresItemsFilterInput>;
};

type SchemasJsonPropertiesCargo_featuresItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesCargo_featuresItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesDependencies = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesDependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesDependenciesX_taplo>;
};

type SchemasJsonPropertiesDependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesDependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesDependenciesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesDependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesDependenciesX_taploFilterInput>;
};

type SchemasJsonPropertiesDependenciesX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesDependenciesX_taploLinks>;
};

type SchemasJsonPropertiesDependenciesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesDependenciesX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesDependenciesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesDependenciesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesDev_dependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesDev_dependenciesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesDev_dependenciesX_taplo>;
};

type SchemasJsonPropertiesDev_dependenciesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesDev_dependenciesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesDev_dependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesDev_dependenciesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesDev_dependenciesX_taploFilterInput>;
};

type SchemasJsonPropertiesDev_dependenciesX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonPropertiesDev_dependenciesX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesEdition = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Date']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Date']>>>;
};


type SchemasJsonPropertiesEdition_defaultArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};


type SchemasJsonPropertiesEdition_enumArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesEditionFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<DateQueryOperatorInput>;
  readonly enum: Maybe<DateQueryOperatorInput>;
};

type SchemasJsonPropertiesExample = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesExampleItems>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesExampleX_taplo>;
};

type SchemasJsonPropertiesExampleFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesExampleItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesExampleX_taploFilterInput>;
};

type SchemasJsonPropertiesExampleItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesExampleItemsX_taplo>;
};

type SchemasJsonPropertiesExampleItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesExampleItemsX_taploFilterInput>;
};

type SchemasJsonPropertiesExampleItemsX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesExampleItemsX_taploLinks>;
};

type SchemasJsonPropertiesExampleItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesExampleItemsX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesExampleItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesExampleItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesExampleX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesExampleX_taploLinks>;
};

type SchemasJsonPropertiesExampleX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesExampleX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesExampleX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesExampleX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesFeatures = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesFeaturesAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesFeaturesX_taplo>;
};

type SchemasJsonPropertiesFeaturesAdditionalProperties = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesFeaturesAdditionalPropertiesItems>;
};

type SchemasJsonPropertiesFeaturesAdditionalPropertiesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesFeaturesAdditionalPropertiesItemsFilterInput>;
};

type SchemasJsonPropertiesFeaturesAdditionalPropertiesItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesFeaturesAdditionalPropertiesItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesFeaturesFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesFeaturesAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesFeaturesX_taploFilterInput>;
};

type SchemasJsonPropertiesFeaturesX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesFeaturesX_taploLinks>;
};

type SchemasJsonPropertiesFeaturesX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesFeaturesX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesFeaturesX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesFeaturesX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesFilterInput = {
  readonly tab_spaces: Maybe<SchemasJsonPropertiesTab_spacesFilterInput>;
  readonly fn_args_layout: Maybe<SchemasJsonPropertiesFn_args_layoutFilterInput>;
  readonly merge_derives: Maybe<SchemasJsonPropertiesMerge_derivesFilterInput>;
  readonly print_misformatted_file_names: Maybe<SchemasJsonPropertiesPrint_misformatted_file_namesFilterInput>;
  readonly remove_nested_parens: Maybe<SchemasJsonPropertiesRemove_nested_parensFilterInput>;
  readonly use_small_heuristics: Maybe<SchemasJsonPropertiesUse_small_heuristicsFilterInput>;
  readonly use_try_shorthand: Maybe<SchemasJsonPropertiesUse_try_shorthandFilterInput>;
  readonly reorder_modules: Maybe<SchemasJsonPropertiesReorder_modulesFilterInput>;
  readonly hard_tabs: Maybe<SchemasJsonPropertiesHard_tabsFilterInput>;
  readonly use_field_init_shorthand: Maybe<SchemasJsonPropertiesUse_field_init_shorthandFilterInput>;
  readonly max_width: Maybe<SchemasJsonPropertiesMax_widthFilterInput>;
  readonly reorder_imports: Maybe<SchemasJsonPropertiesReorder_importsFilterInput>;
  readonly match_arm_leading_pipes: Maybe<SchemasJsonPropertiesMatch_arm_leading_pipesFilterInput>;
  readonly force_explicit_abi: Maybe<SchemasJsonPropertiesForce_explicit_abiFilterInput>;
  readonly edition: Maybe<SchemasJsonPropertiesEditionFilterInput>;
  readonly newline_style: Maybe<SchemasJsonPropertiesNewline_styleFilterInput>;
  readonly build_system: Maybe<SchemasJsonPropertiesBuild_systemFilterInput>;
  readonly tool: Maybe<SchemasJsonPropertiesToolFilterInput>;
  readonly badges: Maybe<SchemasJsonPropertiesBadgesFilterInput>;
  readonly bench: Maybe<SchemasJsonPropertiesBenchFilterInput>;
  readonly bin: Maybe<SchemasJsonPropertiesBinFilterInput>;
  readonly build_dependencies: Maybe<SchemasJsonPropertiesBuild_dependenciesFilterInput>;
  readonly cargo_features: Maybe<SchemasJsonPropertiesCargo_featuresFilterInput>;
  readonly dependencies: Maybe<SchemasJsonPropertiesDependenciesFilterInput>;
  readonly dev_dependencies: Maybe<SchemasJsonPropertiesDev_dependenciesFilterInput>;
  readonly example: Maybe<SchemasJsonPropertiesExampleFilterInput>;
  readonly features: Maybe<SchemasJsonPropertiesFeaturesFilterInput>;
  readonly lib: Maybe<SchemasJsonPropertiesLibFilterInput>;
  readonly package: Maybe<SchemasJsonPropertiesPackageFilterInput>;
  readonly patch: Maybe<SchemasJsonPropertiesPatchFilterInput>;
  readonly profile: Maybe<SchemasJsonPropertiesProfileFilterInput>;
  readonly project: Maybe<SchemasJsonPropertiesProjectFilterInput>;
  readonly replace: Maybe<SchemasJsonPropertiesReplaceFilterInput>;
  readonly target: Maybe<SchemasJsonPropertiesTargetFilterInput>;
  readonly test: Maybe<SchemasJsonPropertiesTestFilterInput>;
  readonly workspace: Maybe<SchemasJsonPropertiesWorkspaceFilterInput>;
};

type SchemasJsonPropertiesFn_args_layout = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonPropertiesFn_args_layoutFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesForce_explicit_abi = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesForce_explicit_abiFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesHard_tabs = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesHard_tabsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesLib = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesLibX_taplo>;
};

type SchemasJsonPropertiesLibFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesLibX_taploFilterInput>;
};

type SchemasJsonPropertiesLibX_taplo = {
  readonly docs: Maybe<SchemasJsonPropertiesLibX_taploDocs>;
  readonly links: Maybe<SchemasJsonPropertiesLibX_taploLinks>;
};

type SchemasJsonPropertiesLibX_taploDocs = {
  readonly main: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesLibX_taploDocsFilterInput = {
  readonly main: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesLibX_taploFilterInput = {
  readonly docs: Maybe<SchemasJsonPropertiesLibX_taploDocsFilterInput>;
  readonly links: Maybe<SchemasJsonPropertiesLibX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesLibX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesLibX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesMatch_arm_leading_pipes = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonPropertiesMatch_arm_leading_pipesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesMax_width = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Int']>;
};

type SchemasJsonPropertiesMax_widthFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<IntQueryOperatorInput>;
};

type SchemasJsonPropertiesMerge_derives = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesMerge_derivesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesNewline_style = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonPropertiesNewline_styleFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesPackage = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesPackageFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesPatch = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesPatchAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesPatchX_taplo>;
};

type SchemasJsonPropertiesPatchAdditionalProperties = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesPatchAdditionalPropertiesAdditionalProperties>;
};

type SchemasJsonPropertiesPatchAdditionalPropertiesAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesPatchAdditionalPropertiesAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesPatchAdditionalPropertiesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesPatchAdditionalPropertiesAdditionalPropertiesFilterInput>;
};

type SchemasJsonPropertiesPatchFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesPatchAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesPatchX_taploFilterInput>;
};

type SchemasJsonPropertiesPatchX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesPatchX_taploLinks>;
};

type SchemasJsonPropertiesPatchX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesPatchX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesPatchX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesPatchX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesPrint_misformatted_file_names = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesPrint_misformatted_file_namesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesProfile = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesProfileFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesProject = {
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesProjectX_taplo>;
};

type SchemasJsonPropertiesProjectFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesProjectX_taploFilterInput>;
};

type SchemasJsonPropertiesProjectX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonPropertiesProjectX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesRemove_nested_parens = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesRemove_nested_parensFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesReorder_imports = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesReorder_importsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesReorder_modules = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesReorder_modulesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesReplace = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesReplaceAdditionalProperties>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesReplaceX_taplo>;
};

type SchemasJsonPropertiesReplaceAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesReplaceAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesReplaceFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesReplaceAdditionalPropertiesFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesReplaceX_taploFilterInput>;
};

type SchemasJsonPropertiesReplaceX_taplo = {
  readonly hidden: Maybe<Scalars['Boolean']>;
};

type SchemasJsonPropertiesReplaceX_taploFilterInput = {
  readonly hidden: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesTab_spaces = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Int']>;
};

type SchemasJsonPropertiesTab_spacesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<IntQueryOperatorInput>;
};

type SchemasJsonPropertiesTarget = {
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesTargetAdditionalProperties>;
};

type SchemasJsonPropertiesTargetAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesTargetAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesTargetFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesTargetAdditionalPropertiesFilterInput>;
};

type SchemasJsonPropertiesTest = {
  readonly description: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesTestItems>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesTestX_taplo>;
};

type SchemasJsonPropertiesTestFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesTestItemsFilterInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesTestX_taploFilterInput>;
};

type SchemasJsonPropertiesTestItems = {
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesTestItemsX_taplo>;
};

type SchemasJsonPropertiesTestItemsFilterInput = {
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesTestItemsX_taploFilterInput>;
};

type SchemasJsonPropertiesTestItemsX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesTestItemsX_taploLinks>;
};

type SchemasJsonPropertiesTestItemsX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesTestItemsX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesTestItemsX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesTestItemsX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesTestX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesTestX_taploLinks>;
};

type SchemasJsonPropertiesTestX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesTestX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesTestX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesTestX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesTool = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesToolX_taplo>;
  readonly properties: Maybe<SchemasJsonPropertiesToolProperties>;
};

type SchemasJsonPropertiesToolFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly x_taplo: Maybe<SchemasJsonPropertiesToolX_taploFilterInput>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolProperties = {
  readonly poetry: Maybe<SchemasJsonPropertiesToolPropertiesPoetry>;
};

type SchemasJsonPropertiesToolPropertiesFilterInput = {
  readonly poetry: Maybe<SchemasJsonPropertiesToolPropertiesPoetryFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetry = {
  readonly name: Maybe<Scalars['String']>;
  readonly type: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryFilterInput = {
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryProperties = {
  readonly name: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesName>;
  readonly version: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesVersion>;
  readonly description: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDescription>;
  readonly keywords: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywords>;
  readonly homepage: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesHomepage>;
  readonly repository: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesRepository>;
  readonly documentation: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDocumentation>;
  readonly license: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesLicense>;
  readonly authors: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesAuthors>;
  readonly maintainers: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesMaintainers>;
  readonly readme: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesReadme>;
  readonly classifiers: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesClassifiers>;
  readonly packages: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackages>;
  readonly include: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesInclude>;
  readonly exclude: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExclude>;
  readonly dependencies: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependencies>;
  readonly dev_dependencies: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependencies>;
  readonly extras: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtras>;
  readonly build: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesBuild>;
  readonly source: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSource>;
  readonly scripts: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesScripts>;
  readonly plugins: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPlugins>;
  readonly urls: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrls>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesAuthors = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesAuthorsFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesBuild = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesBuildFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesClassifiers = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesClassifiersFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesProperties>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesFilterInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesProperties = {
  readonly python: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesPython>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesFilterInput = {
  readonly python: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesPythonFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesPython = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesPropertiesPythonFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDescription = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDescriptionFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependencies = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDocumentation = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesDocumentationFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExclude = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExcludeFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtras = {
  readonly type: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesFilterInput = {
  readonly name: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesNameFilterInput>;
  readonly version: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesVersionFilterInput>;
  readonly description: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDescriptionFilterInput>;
  readonly keywords: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsFilterInput>;
  readonly homepage: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesHomepageFilterInput>;
  readonly repository: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesRepositoryFilterInput>;
  readonly documentation: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDocumentationFilterInput>;
  readonly license: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesLicenseFilterInput>;
  readonly authors: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesAuthorsFilterInput>;
  readonly maintainers: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesMaintainersFilterInput>;
  readonly readme: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesReadmeFilterInput>;
  readonly classifiers: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesClassifiersFilterInput>;
  readonly packages: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesFilterInput>;
  readonly include: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesIncludeFilterInput>;
  readonly exclude: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExcludeFilterInput>;
  readonly dependencies: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDependenciesFilterInput>;
  readonly dev_dependencies: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesDev_dependenciesFilterInput>;
  readonly extras: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesExtrasFilterInput>;
  readonly build: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesBuildFilterInput>;
  readonly source: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceFilterInput>;
  readonly scripts: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsFilterInput>;
  readonly plugins: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsFilterInput>;
  readonly urls: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesHomepage = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesHomepageFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesInclude = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesIncludeFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywords = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsItems = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesKeywordsItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesLicense = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesLicenseFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesMaintainers = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesMaintainersFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesName = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesNameFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackages = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItems = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<Scalars['Boolean']>;
  readonly required: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<BooleanQueryOperatorInput>;
  readonly required: Maybe<StringQueryOperatorInput>;
  readonly properties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsProperties = {
  readonly include: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesInclude>;
  readonly from: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFrom>;
  readonly format: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormat>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFilterInput = {
  readonly include: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesIncludeFilterInput>;
  readonly from: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFromFilterInput>;
  readonly format: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormat = {
  readonly oneOf: Maybe<ReadonlyArray<Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOf>>>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatFilterInput = {
  readonly oneOf: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfFilterListInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOf = {
  readonly type: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfFilterListInput = {
  readonly elemMatch: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFormatOneOfItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFrom = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesFromFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesInclude = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPackagesItemsPropertiesIncludeFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPlugins = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly type: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties_xaxzAxZxxx0x9xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties_xaxzAxZxxx0x9xxx = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties_xaxzAxZxxx0x9xxxFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternPropertiesFilterInput = {
  readonly _xaxzAxZxxx0x9xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesPluginsPatternProperties_xaxzAxZxxx0x9xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesReadme = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesReadmeFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesRepository = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly format: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesRepositoryFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly format: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesScripts = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsItems = {
  readonly type: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesScriptsItemsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSource = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceAdditionalProperties>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceItems>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceAdditionalProperties = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceAdditionalPropertiesFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly additionalProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceAdditionalPropertiesFilterInput>;
  readonly items: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceItemsFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceItems = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesSourceItemsFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrls = {
  readonly type: Maybe<Scalars['String']>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly patternProperties: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternPropertiesFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties = {
  readonly _xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties_xxx>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties_xxx = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties_xxxFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternPropertiesFilterInput = {
  readonly _xxx: Maybe<SchemasJsonPropertiesToolPropertiesPoetryPropertiesUrlsPatternProperties_xxxFilterInput>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesVersion = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolPropertiesPoetryPropertiesVersionFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesToolX_taplo = {
  readonly links: Maybe<SchemasJsonPropertiesToolX_taploLinks>;
};

type SchemasJsonPropertiesToolX_taploFilterInput = {
  readonly links: Maybe<SchemasJsonPropertiesToolX_taploLinksFilterInput>;
};

type SchemasJsonPropertiesToolX_taploLinks = {
  readonly key: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesToolX_taploLinksFilterInput = {
  readonly key: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesUse_field_init_shorthand = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesUse_field_init_shorthandFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesUse_small_heuristics = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['String']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonPropertiesUse_small_heuristicsFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<StringQueryOperatorInput>;
  readonly enum: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonPropertiesUse_try_shorthand = {
  readonly type: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly default: Maybe<Scalars['Boolean']>;
  readonly enum: Maybe<ReadonlyArray<Maybe<Scalars['Boolean']>>>;
};

type SchemasJsonPropertiesUse_try_shorthandFilterInput = {
  readonly type: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly default: Maybe<BooleanQueryOperatorInput>;
  readonly enum: Maybe<BooleanQueryOperatorInput>;
};

type SchemasJsonPropertiesWorkspace = {
  readonly _ref: Maybe<Scalars['String']>;
};

type SchemasJsonPropertiesWorkspaceFilterInput = {
  readonly _ref: Maybe<StringQueryOperatorInput>;
};

type SchemasJsonSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<SchemasJsonFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type SchemasJsonX_taplo_info = {
  readonly authors: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly patterns: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SchemasJsonX_taplo_infoFilterInput = {
  readonly authors: Maybe<StringQueryOperatorInput>;
  readonly patterns: Maybe<StringQueryOperatorInput>;
};

type Site = Node & {
  readonly buildTime: Maybe<Scalars['Date']>;
  readonly siteMetadata: Maybe<SiteSiteMetadata>;
  readonly polyfill: Maybe<Scalars['Boolean']>;
  readonly pathPrefix: Maybe<Scalars['String']>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
};


type Site_buildTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type SiteBuildMetadata = Node & {
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
  readonly buildTime: Maybe<Scalars['Date']>;
};


type SiteBuildMetadata_buildTimeArgs = {
  formatString: Maybe<Scalars['String']>;
  fromNow: Maybe<Scalars['Boolean']>;
  difference: Maybe<Scalars['String']>;
  locale: Maybe<Scalars['String']>;
};

type SiteBuildMetadataConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SiteBuildMetadataEdge>;
  readonly nodes: ReadonlyArray<SiteBuildMetadata>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<SiteBuildMetadataGroupConnection>;
};


type SiteBuildMetadataConnection_distinctArgs = {
  field: SiteBuildMetadataFieldsEnum;
};


type SiteBuildMetadataConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: SiteBuildMetadataFieldsEnum;
};

type SiteBuildMetadataEdge = {
  readonly next: Maybe<SiteBuildMetadata>;
  readonly node: SiteBuildMetadata;
  readonly previous: Maybe<SiteBuildMetadata>;
};

enum SiteBuildMetadataFieldsEnum {
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type',
  buildTime = 'buildTime'
}

type SiteBuildMetadataFilterInput = {
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
  readonly buildTime: Maybe<DateQueryOperatorInput>;
};

type SiteBuildMetadataGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SiteBuildMetadataEdge>;
  readonly nodes: ReadonlyArray<SiteBuildMetadata>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type SiteBuildMetadataSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<SiteBuildMetadataFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type SiteConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SiteEdge>;
  readonly nodes: ReadonlyArray<Site>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<SiteGroupConnection>;
};


type SiteConnection_distinctArgs = {
  field: SiteFieldsEnum;
};


type SiteConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: SiteFieldsEnum;
};

type SiteEdge = {
  readonly next: Maybe<Site>;
  readonly node: Site;
  readonly previous: Maybe<Site>;
};

enum SiteFieldsEnum {
  buildTime = 'buildTime',
  siteMetadata___title = 'siteMetadata.title',
  siteMetadata___description = 'siteMetadata.description',
  siteMetadata___siteUrl = 'siteMetadata.siteUrl',
  polyfill = 'polyfill',
  pathPrefix = 'pathPrefix',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type'
}

type SiteFilterInput = {
  readonly buildTime: Maybe<DateQueryOperatorInput>;
  readonly siteMetadata: Maybe<SiteSiteMetadataFilterInput>;
  readonly polyfill: Maybe<BooleanQueryOperatorInput>;
  readonly pathPrefix: Maybe<StringQueryOperatorInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
};

type SiteGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SiteEdge>;
  readonly nodes: ReadonlyArray<Site>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type SitePage = Node & {
  readonly path: Scalars['String'];
  readonly component: Scalars['String'];
  readonly internalComponentName: Scalars['String'];
  readonly componentChunkName: Scalars['String'];
  readonly matchPath: Maybe<Scalars['String']>;
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
  readonly isCreatedByStatefulCreatePages: Maybe<Scalars['Boolean']>;
  readonly context: Maybe<SitePageContext>;
  readonly pluginCreator: Maybe<SitePlugin>;
  readonly pluginCreatorId: Maybe<Scalars['String']>;
  readonly componentPath: Maybe<Scalars['String']>;
};

type SitePageConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SitePageEdge>;
  readonly nodes: ReadonlyArray<SitePage>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<SitePageGroupConnection>;
};


type SitePageConnection_distinctArgs = {
  field: SitePageFieldsEnum;
};


type SitePageConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: SitePageFieldsEnum;
};

type SitePageContext = {
  readonly frontmatter: Maybe<SitePageContextFrontmatter>;
};

type SitePageContextFilterInput = {
  readonly frontmatter: Maybe<SitePageContextFrontmatterFilterInput>;
};

type SitePageContextFrontmatter = {
  readonly title: Maybe<Scalars['String']>;
  readonly nav: Maybe<Scalars['String']>;
  readonly navOrder: Maybe<Scalars['Int']>;
};

type SitePageContextFrontmatterFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly nav: Maybe<StringQueryOperatorInput>;
  readonly navOrder: Maybe<IntQueryOperatorInput>;
};

type SitePageEdge = {
  readonly next: Maybe<SitePage>;
  readonly node: SitePage;
  readonly previous: Maybe<SitePage>;
};

enum SitePageFieldsEnum {
  path = 'path',
  component = 'component',
  internalComponentName = 'internalComponentName',
  componentChunkName = 'componentChunkName',
  matchPath = 'matchPath',
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type',
  isCreatedByStatefulCreatePages = 'isCreatedByStatefulCreatePages',
  context___frontmatter___title = 'context.frontmatter.title',
  context___frontmatter___nav = 'context.frontmatter.nav',
  context___frontmatter___navOrder = 'context.frontmatter.navOrder',
  pluginCreator___id = 'pluginCreator.id',
  pluginCreator___parent___id = 'pluginCreator.parent.id',
  pluginCreator___parent___parent___id = 'pluginCreator.parent.parent.id',
  pluginCreator___parent___parent___children = 'pluginCreator.parent.parent.children',
  pluginCreator___parent___children = 'pluginCreator.parent.children',
  pluginCreator___parent___children___id = 'pluginCreator.parent.children.id',
  pluginCreator___parent___children___children = 'pluginCreator.parent.children.children',
  pluginCreator___parent___internal___content = 'pluginCreator.parent.internal.content',
  pluginCreator___parent___internal___contentDigest = 'pluginCreator.parent.internal.contentDigest',
  pluginCreator___parent___internal___description = 'pluginCreator.parent.internal.description',
  pluginCreator___parent___internal___fieldOwners = 'pluginCreator.parent.internal.fieldOwners',
  pluginCreator___parent___internal___ignoreType = 'pluginCreator.parent.internal.ignoreType',
  pluginCreator___parent___internal___mediaType = 'pluginCreator.parent.internal.mediaType',
  pluginCreator___parent___internal___owner = 'pluginCreator.parent.internal.owner',
  pluginCreator___parent___internal___type = 'pluginCreator.parent.internal.type',
  pluginCreator___children = 'pluginCreator.children',
  pluginCreator___children___id = 'pluginCreator.children.id',
  pluginCreator___children___parent___id = 'pluginCreator.children.parent.id',
  pluginCreator___children___parent___children = 'pluginCreator.children.parent.children',
  pluginCreator___children___children = 'pluginCreator.children.children',
  pluginCreator___children___children___id = 'pluginCreator.children.children.id',
  pluginCreator___children___children___children = 'pluginCreator.children.children.children',
  pluginCreator___children___internal___content = 'pluginCreator.children.internal.content',
  pluginCreator___children___internal___contentDigest = 'pluginCreator.children.internal.contentDigest',
  pluginCreator___children___internal___description = 'pluginCreator.children.internal.description',
  pluginCreator___children___internal___fieldOwners = 'pluginCreator.children.internal.fieldOwners',
  pluginCreator___children___internal___ignoreType = 'pluginCreator.children.internal.ignoreType',
  pluginCreator___children___internal___mediaType = 'pluginCreator.children.internal.mediaType',
  pluginCreator___children___internal___owner = 'pluginCreator.children.internal.owner',
  pluginCreator___children___internal___type = 'pluginCreator.children.internal.type',
  pluginCreator___internal___content = 'pluginCreator.internal.content',
  pluginCreator___internal___contentDigest = 'pluginCreator.internal.contentDigest',
  pluginCreator___internal___description = 'pluginCreator.internal.description',
  pluginCreator___internal___fieldOwners = 'pluginCreator.internal.fieldOwners',
  pluginCreator___internal___ignoreType = 'pluginCreator.internal.ignoreType',
  pluginCreator___internal___mediaType = 'pluginCreator.internal.mediaType',
  pluginCreator___internal___owner = 'pluginCreator.internal.owner',
  pluginCreator___internal___type = 'pluginCreator.internal.type',
  pluginCreator___resolve = 'pluginCreator.resolve',
  pluginCreator___name = 'pluginCreator.name',
  pluginCreator___version = 'pluginCreator.version',
  pluginCreator___pluginOptions___implementation___info = 'pluginCreator.pluginOptions.implementation.info',
  pluginCreator___pluginOptions___indentedSyntax = 'pluginCreator.pluginOptions.indentedSyntax',
  pluginCreator___pluginOptions___indentType = 'pluginCreator.pluginOptions.indentType',
  pluginCreator___pluginOptions___indentWidth = 'pluginCreator.pluginOptions.indentWidth',
  pluginCreator___pluginOptions___linefeed = 'pluginCreator.pluginOptions.linefeed',
  pluginCreator___pluginOptions___omitSourceMapUrl = 'pluginCreator.pluginOptions.omitSourceMapUrl',
  pluginCreator___pluginOptions___precision = 'pluginCreator.pluginOptions.precision',
  pluginCreator___pluginOptions___sourceComments = 'pluginCreator.pluginOptions.sourceComments',
  pluginCreator___pluginOptions___sourceMapContents = 'pluginCreator.pluginOptions.sourceMapContents',
  pluginCreator___pluginOptions___sourceMapEmbed = 'pluginCreator.pluginOptions.sourceMapEmbed',
  pluginCreator___pluginOptions___base64Width = 'pluginCreator.pluginOptions.base64Width',
  pluginCreator___pluginOptions___stripMetadata = 'pluginCreator.pluginOptions.stripMetadata',
  pluginCreator___pluginOptions___defaultQuality = 'pluginCreator.pluginOptions.defaultQuality',
  pluginCreator___pluginOptions___failOnError = 'pluginCreator.pluginOptions.failOnError',
  pluginCreator___pluginOptions___output = 'pluginCreator.pluginOptions.output',
  pluginCreator___pluginOptions___createLinkInHead = 'pluginCreator.pluginOptions.createLinkInHead',
  pluginCreator___pluginOptions___icon = 'pluginCreator.pluginOptions.icon',
  pluginCreator___pluginOptions___legacy = 'pluginCreator.pluginOptions.legacy',
  pluginCreator___pluginOptions___theme_color_in_head = 'pluginCreator.pluginOptions.theme_color_in_head',
  pluginCreator___pluginOptions___cache_busting_mode = 'pluginCreator.pluginOptions.cache_busting_mode',
  pluginCreator___pluginOptions___crossOrigin = 'pluginCreator.pluginOptions.crossOrigin',
  pluginCreator___pluginOptions___include_favicon = 'pluginCreator.pluginOptions.include_favicon',
  pluginCreator___pluginOptions___cacheDigest = 'pluginCreator.pluginOptions.cacheDigest',
  pluginCreator___pluginOptions___defaultLayouts___default = 'pluginCreator.pluginOptions.defaultLayouts.default',
  pluginCreator___pluginOptions___extensions = 'pluginCreator.pluginOptions.extensions',
  pluginCreator___pluginOptions___lessBabel = 'pluginCreator.pluginOptions.lessBabel',
  pluginCreator___pluginOptions___mediaTypes = 'pluginCreator.pluginOptions.mediaTypes',
  pluginCreator___pluginOptions___name = 'pluginCreator.pluginOptions.name',
  pluginCreator___pluginOptions___path = 'pluginCreator.pluginOptions.path',
  pluginCreator___pluginOptions___style = 'pluginCreator.pluginOptions.style',
  pluginCreator___pluginOptions___lessOptions___javascriptEnabled = 'pluginCreator.pluginOptions.lessOptions.javascriptEnabled',
  pluginCreator___pluginOptions___pathCheck = 'pluginCreator.pluginOptions.pathCheck',
  pluginCreator___pluginOptions___allExtensions = 'pluginCreator.pluginOptions.allExtensions',
  pluginCreator___pluginOptions___isTSX = 'pluginCreator.pluginOptions.isTSX',
  pluginCreator___pluginOptions___jsxPragma = 'pluginCreator.pluginOptions.jsxPragma',
  pluginCreator___nodeAPIs = 'pluginCreator.nodeAPIs',
  pluginCreator___browserAPIs = 'pluginCreator.browserAPIs',
  pluginCreator___ssrAPIs = 'pluginCreator.ssrAPIs',
  pluginCreator___pluginFilepath = 'pluginCreator.pluginFilepath',
  pluginCreator___packageJson___name = 'pluginCreator.packageJson.name',
  pluginCreator___packageJson___description = 'pluginCreator.packageJson.description',
  pluginCreator___packageJson___version = 'pluginCreator.packageJson.version',
  pluginCreator___packageJson___main = 'pluginCreator.packageJson.main',
  pluginCreator___packageJson___license = 'pluginCreator.packageJson.license',
  pluginCreator___packageJson___dependencies = 'pluginCreator.packageJson.dependencies',
  pluginCreator___packageJson___dependencies___name = 'pluginCreator.packageJson.dependencies.name',
  pluginCreator___packageJson___dependencies___version = 'pluginCreator.packageJson.dependencies.version',
  pluginCreator___packageJson___devDependencies = 'pluginCreator.packageJson.devDependencies',
  pluginCreator___packageJson___devDependencies___name = 'pluginCreator.packageJson.devDependencies.name',
  pluginCreator___packageJson___devDependencies___version = 'pluginCreator.packageJson.devDependencies.version',
  pluginCreator___packageJson___peerDependencies = 'pluginCreator.packageJson.peerDependencies',
  pluginCreator___packageJson___peerDependencies___name = 'pluginCreator.packageJson.peerDependencies.name',
  pluginCreator___packageJson___peerDependencies___version = 'pluginCreator.packageJson.peerDependencies.version',
  pluginCreator___packageJson___keywords = 'pluginCreator.packageJson.keywords',
  pluginCreatorId = 'pluginCreatorId',
  componentPath = 'componentPath'
}

type SitePageFilterInput = {
  readonly path: Maybe<StringQueryOperatorInput>;
  readonly component: Maybe<StringQueryOperatorInput>;
  readonly internalComponentName: Maybe<StringQueryOperatorInput>;
  readonly componentChunkName: Maybe<StringQueryOperatorInput>;
  readonly matchPath: Maybe<StringQueryOperatorInput>;
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
  readonly isCreatedByStatefulCreatePages: Maybe<BooleanQueryOperatorInput>;
  readonly context: Maybe<SitePageContextFilterInput>;
  readonly pluginCreator: Maybe<SitePluginFilterInput>;
  readonly pluginCreatorId: Maybe<StringQueryOperatorInput>;
  readonly componentPath: Maybe<StringQueryOperatorInput>;
};

type SitePageGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SitePageEdge>;
  readonly nodes: ReadonlyArray<SitePage>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type SitePageSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<SitePageFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type SitePlugin = Node & {
  readonly id: Scalars['ID'];
  readonly parent: Maybe<Node>;
  readonly children: ReadonlyArray<Node>;
  readonly internal: Internal;
  readonly resolve: Maybe<Scalars['String']>;
  readonly name: Maybe<Scalars['String']>;
  readonly version: Maybe<Scalars['String']>;
  readonly pluginOptions: Maybe<SitePluginPluginOptions>;
  readonly nodeAPIs: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly browserAPIs: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly ssrAPIs: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly pluginFilepath: Maybe<Scalars['String']>;
  readonly packageJson: Maybe<SitePluginPackageJson>;
};

type SitePluginConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SitePluginEdge>;
  readonly nodes: ReadonlyArray<SitePlugin>;
  readonly pageInfo: PageInfo;
  readonly distinct: ReadonlyArray<Scalars['String']>;
  readonly group: ReadonlyArray<SitePluginGroupConnection>;
};


type SitePluginConnection_distinctArgs = {
  field: SitePluginFieldsEnum;
};


type SitePluginConnection_groupArgs = {
  skip: Maybe<Scalars['Int']>;
  limit: Maybe<Scalars['Int']>;
  field: SitePluginFieldsEnum;
};

type SitePluginEdge = {
  readonly next: Maybe<SitePlugin>;
  readonly node: SitePlugin;
  readonly previous: Maybe<SitePlugin>;
};

enum SitePluginFieldsEnum {
  id = 'id',
  parent___id = 'parent.id',
  parent___parent___id = 'parent.parent.id',
  parent___parent___parent___id = 'parent.parent.parent.id',
  parent___parent___parent___children = 'parent.parent.parent.children',
  parent___parent___children = 'parent.parent.children',
  parent___parent___children___id = 'parent.parent.children.id',
  parent___parent___children___children = 'parent.parent.children.children',
  parent___parent___internal___content = 'parent.parent.internal.content',
  parent___parent___internal___contentDigest = 'parent.parent.internal.contentDigest',
  parent___parent___internal___description = 'parent.parent.internal.description',
  parent___parent___internal___fieldOwners = 'parent.parent.internal.fieldOwners',
  parent___parent___internal___ignoreType = 'parent.parent.internal.ignoreType',
  parent___parent___internal___mediaType = 'parent.parent.internal.mediaType',
  parent___parent___internal___owner = 'parent.parent.internal.owner',
  parent___parent___internal___type = 'parent.parent.internal.type',
  parent___children = 'parent.children',
  parent___children___id = 'parent.children.id',
  parent___children___parent___id = 'parent.children.parent.id',
  parent___children___parent___children = 'parent.children.parent.children',
  parent___children___children = 'parent.children.children',
  parent___children___children___id = 'parent.children.children.id',
  parent___children___children___children = 'parent.children.children.children',
  parent___children___internal___content = 'parent.children.internal.content',
  parent___children___internal___contentDigest = 'parent.children.internal.contentDigest',
  parent___children___internal___description = 'parent.children.internal.description',
  parent___children___internal___fieldOwners = 'parent.children.internal.fieldOwners',
  parent___children___internal___ignoreType = 'parent.children.internal.ignoreType',
  parent___children___internal___mediaType = 'parent.children.internal.mediaType',
  parent___children___internal___owner = 'parent.children.internal.owner',
  parent___children___internal___type = 'parent.children.internal.type',
  parent___internal___content = 'parent.internal.content',
  parent___internal___contentDigest = 'parent.internal.contentDigest',
  parent___internal___description = 'parent.internal.description',
  parent___internal___fieldOwners = 'parent.internal.fieldOwners',
  parent___internal___ignoreType = 'parent.internal.ignoreType',
  parent___internal___mediaType = 'parent.internal.mediaType',
  parent___internal___owner = 'parent.internal.owner',
  parent___internal___type = 'parent.internal.type',
  children = 'children',
  children___id = 'children.id',
  children___parent___id = 'children.parent.id',
  children___parent___parent___id = 'children.parent.parent.id',
  children___parent___parent___children = 'children.parent.parent.children',
  children___parent___children = 'children.parent.children',
  children___parent___children___id = 'children.parent.children.id',
  children___parent___children___children = 'children.parent.children.children',
  children___parent___internal___content = 'children.parent.internal.content',
  children___parent___internal___contentDigest = 'children.parent.internal.contentDigest',
  children___parent___internal___description = 'children.parent.internal.description',
  children___parent___internal___fieldOwners = 'children.parent.internal.fieldOwners',
  children___parent___internal___ignoreType = 'children.parent.internal.ignoreType',
  children___parent___internal___mediaType = 'children.parent.internal.mediaType',
  children___parent___internal___owner = 'children.parent.internal.owner',
  children___parent___internal___type = 'children.parent.internal.type',
  children___children = 'children.children',
  children___children___id = 'children.children.id',
  children___children___parent___id = 'children.children.parent.id',
  children___children___parent___children = 'children.children.parent.children',
  children___children___children = 'children.children.children',
  children___children___children___id = 'children.children.children.id',
  children___children___children___children = 'children.children.children.children',
  children___children___internal___content = 'children.children.internal.content',
  children___children___internal___contentDigest = 'children.children.internal.contentDigest',
  children___children___internal___description = 'children.children.internal.description',
  children___children___internal___fieldOwners = 'children.children.internal.fieldOwners',
  children___children___internal___ignoreType = 'children.children.internal.ignoreType',
  children___children___internal___mediaType = 'children.children.internal.mediaType',
  children___children___internal___owner = 'children.children.internal.owner',
  children___children___internal___type = 'children.children.internal.type',
  children___internal___content = 'children.internal.content',
  children___internal___contentDigest = 'children.internal.contentDigest',
  children___internal___description = 'children.internal.description',
  children___internal___fieldOwners = 'children.internal.fieldOwners',
  children___internal___ignoreType = 'children.internal.ignoreType',
  children___internal___mediaType = 'children.internal.mediaType',
  children___internal___owner = 'children.internal.owner',
  children___internal___type = 'children.internal.type',
  internal___content = 'internal.content',
  internal___contentDigest = 'internal.contentDigest',
  internal___description = 'internal.description',
  internal___fieldOwners = 'internal.fieldOwners',
  internal___ignoreType = 'internal.ignoreType',
  internal___mediaType = 'internal.mediaType',
  internal___owner = 'internal.owner',
  internal___type = 'internal.type',
  resolve = 'resolve',
  name = 'name',
  version = 'version',
  pluginOptions___implementation___info = 'pluginOptions.implementation.info',
  pluginOptions___indentedSyntax = 'pluginOptions.indentedSyntax',
  pluginOptions___indentType = 'pluginOptions.indentType',
  pluginOptions___indentWidth = 'pluginOptions.indentWidth',
  pluginOptions___linefeed = 'pluginOptions.linefeed',
  pluginOptions___omitSourceMapUrl = 'pluginOptions.omitSourceMapUrl',
  pluginOptions___precision = 'pluginOptions.precision',
  pluginOptions___sourceComments = 'pluginOptions.sourceComments',
  pluginOptions___sourceMapContents = 'pluginOptions.sourceMapContents',
  pluginOptions___sourceMapEmbed = 'pluginOptions.sourceMapEmbed',
  pluginOptions___base64Width = 'pluginOptions.base64Width',
  pluginOptions___stripMetadata = 'pluginOptions.stripMetadata',
  pluginOptions___defaultQuality = 'pluginOptions.defaultQuality',
  pluginOptions___failOnError = 'pluginOptions.failOnError',
  pluginOptions___output = 'pluginOptions.output',
  pluginOptions___createLinkInHead = 'pluginOptions.createLinkInHead',
  pluginOptions___icon = 'pluginOptions.icon',
  pluginOptions___legacy = 'pluginOptions.legacy',
  pluginOptions___theme_color_in_head = 'pluginOptions.theme_color_in_head',
  pluginOptions___cache_busting_mode = 'pluginOptions.cache_busting_mode',
  pluginOptions___crossOrigin = 'pluginOptions.crossOrigin',
  pluginOptions___include_favicon = 'pluginOptions.include_favicon',
  pluginOptions___cacheDigest = 'pluginOptions.cacheDigest',
  pluginOptions___defaultLayouts___default = 'pluginOptions.defaultLayouts.default',
  pluginOptions___extensions = 'pluginOptions.extensions',
  pluginOptions___lessBabel = 'pluginOptions.lessBabel',
  pluginOptions___mediaTypes = 'pluginOptions.mediaTypes',
  pluginOptions___name = 'pluginOptions.name',
  pluginOptions___path = 'pluginOptions.path',
  pluginOptions___style = 'pluginOptions.style',
  pluginOptions___lessOptions___modifyVars___primary_color = 'pluginOptions.lessOptions.modifyVars.primary_color',
  pluginOptions___lessOptions___javascriptEnabled = 'pluginOptions.lessOptions.javascriptEnabled',
  pluginOptions___pathCheck = 'pluginOptions.pathCheck',
  pluginOptions___allExtensions = 'pluginOptions.allExtensions',
  pluginOptions___isTSX = 'pluginOptions.isTSX',
  pluginOptions___jsxPragma = 'pluginOptions.jsxPragma',
  nodeAPIs = 'nodeAPIs',
  browserAPIs = 'browserAPIs',
  ssrAPIs = 'ssrAPIs',
  pluginFilepath = 'pluginFilepath',
  packageJson___name = 'packageJson.name',
  packageJson___description = 'packageJson.description',
  packageJson___version = 'packageJson.version',
  packageJson___main = 'packageJson.main',
  packageJson___license = 'packageJson.license',
  packageJson___dependencies = 'packageJson.dependencies',
  packageJson___dependencies___name = 'packageJson.dependencies.name',
  packageJson___dependencies___version = 'packageJson.dependencies.version',
  packageJson___devDependencies = 'packageJson.devDependencies',
  packageJson___devDependencies___name = 'packageJson.devDependencies.name',
  packageJson___devDependencies___version = 'packageJson.devDependencies.version',
  packageJson___peerDependencies = 'packageJson.peerDependencies',
  packageJson___peerDependencies___name = 'packageJson.peerDependencies.name',
  packageJson___peerDependencies___version = 'packageJson.peerDependencies.version',
  packageJson___keywords = 'packageJson.keywords'
}

type SitePluginFilterInput = {
  readonly id: Maybe<StringQueryOperatorInput>;
  readonly parent: Maybe<NodeFilterInput>;
  readonly children: Maybe<NodeFilterListInput>;
  readonly internal: Maybe<InternalFilterInput>;
  readonly resolve: Maybe<StringQueryOperatorInput>;
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly version: Maybe<StringQueryOperatorInput>;
  readonly pluginOptions: Maybe<SitePluginPluginOptionsFilterInput>;
  readonly nodeAPIs: Maybe<StringQueryOperatorInput>;
  readonly browserAPIs: Maybe<StringQueryOperatorInput>;
  readonly ssrAPIs: Maybe<StringQueryOperatorInput>;
  readonly pluginFilepath: Maybe<StringQueryOperatorInput>;
  readonly packageJson: Maybe<SitePluginPackageJsonFilterInput>;
};

type SitePluginGroupConnection = {
  readonly totalCount: Scalars['Int'];
  readonly edges: ReadonlyArray<SitePluginEdge>;
  readonly nodes: ReadonlyArray<SitePlugin>;
  readonly pageInfo: PageInfo;
  readonly field: Scalars['String'];
  readonly fieldValue: Maybe<Scalars['String']>;
};

type SitePluginPackageJson = {
  readonly name: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly version: Maybe<Scalars['String']>;
  readonly main: Maybe<Scalars['String']>;
  readonly license: Maybe<Scalars['String']>;
  readonly dependencies: Maybe<ReadonlyArray<Maybe<SitePluginPackageJsonDependencies>>>;
  readonly devDependencies: Maybe<ReadonlyArray<Maybe<SitePluginPackageJsonDevDependencies>>>;
  readonly peerDependencies: Maybe<ReadonlyArray<Maybe<SitePluginPackageJsonPeerDependencies>>>;
  readonly keywords: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
};

type SitePluginPackageJsonDependencies = {
  readonly name: Maybe<Scalars['String']>;
  readonly version: Maybe<Scalars['String']>;
};

type SitePluginPackageJsonDependenciesFilterInput = {
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly version: Maybe<StringQueryOperatorInput>;
};

type SitePluginPackageJsonDependenciesFilterListInput = {
  readonly elemMatch: Maybe<SitePluginPackageJsonDependenciesFilterInput>;
};

type SitePluginPackageJsonDevDependencies = {
  readonly name: Maybe<Scalars['String']>;
  readonly version: Maybe<Scalars['String']>;
};

type SitePluginPackageJsonDevDependenciesFilterInput = {
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly version: Maybe<StringQueryOperatorInput>;
};

type SitePluginPackageJsonDevDependenciesFilterListInput = {
  readonly elemMatch: Maybe<SitePluginPackageJsonDevDependenciesFilterInput>;
};

type SitePluginPackageJsonFilterInput = {
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly version: Maybe<StringQueryOperatorInput>;
  readonly main: Maybe<StringQueryOperatorInput>;
  readonly license: Maybe<StringQueryOperatorInput>;
  readonly dependencies: Maybe<SitePluginPackageJsonDependenciesFilterListInput>;
  readonly devDependencies: Maybe<SitePluginPackageJsonDevDependenciesFilterListInput>;
  readonly peerDependencies: Maybe<SitePluginPackageJsonPeerDependenciesFilterListInput>;
  readonly keywords: Maybe<StringQueryOperatorInput>;
};

type SitePluginPackageJsonPeerDependencies = {
  readonly name: Maybe<Scalars['String']>;
  readonly version: Maybe<Scalars['String']>;
};

type SitePluginPackageJsonPeerDependenciesFilterInput = {
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly version: Maybe<StringQueryOperatorInput>;
};

type SitePluginPackageJsonPeerDependenciesFilterListInput = {
  readonly elemMatch: Maybe<SitePluginPackageJsonPeerDependenciesFilterInput>;
};

type SitePluginPluginOptions = {
  readonly implementation: Maybe<SitePluginPluginOptionsImplementation>;
  readonly indentedSyntax: Maybe<Scalars['Boolean']>;
  readonly indentType: Maybe<Scalars['String']>;
  readonly indentWidth: Maybe<Scalars['Int']>;
  readonly linefeed: Maybe<Scalars['String']>;
  readonly omitSourceMapUrl: Maybe<Scalars['Boolean']>;
  readonly precision: Maybe<Scalars['Int']>;
  readonly sourceComments: Maybe<Scalars['Boolean']>;
  readonly sourceMapContents: Maybe<Scalars['Boolean']>;
  readonly sourceMapEmbed: Maybe<Scalars['Boolean']>;
  readonly base64Width: Maybe<Scalars['Int']>;
  readonly stripMetadata: Maybe<Scalars['Boolean']>;
  readonly defaultQuality: Maybe<Scalars['Int']>;
  readonly failOnError: Maybe<Scalars['Boolean']>;
  readonly output: Maybe<Scalars['String']>;
  readonly createLinkInHead: Maybe<Scalars['Boolean']>;
  readonly icon: Maybe<Scalars['String']>;
  readonly legacy: Maybe<Scalars['Boolean']>;
  readonly theme_color_in_head: Maybe<Scalars['Boolean']>;
  readonly cache_busting_mode: Maybe<Scalars['String']>;
  readonly crossOrigin: Maybe<Scalars['String']>;
  readonly include_favicon: Maybe<Scalars['Boolean']>;
  readonly cacheDigest: Maybe<Scalars['String']>;
  readonly defaultLayouts: Maybe<SitePluginPluginOptionsDefaultLayouts>;
  readonly extensions: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly lessBabel: Maybe<Scalars['Boolean']>;
  readonly mediaTypes: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly name: Maybe<Scalars['String']>;
  readonly path: Maybe<Scalars['String']>;
  readonly style: Maybe<Scalars['Boolean']>;
  readonly lessOptions: Maybe<SitePluginPluginOptionsLessOptions>;
  readonly pathCheck: Maybe<Scalars['Boolean']>;
  readonly allExtensions: Maybe<Scalars['Boolean']>;
  readonly isTSX: Maybe<Scalars['Boolean']>;
  readonly jsxPragma: Maybe<Scalars['String']>;
};

type SitePluginPluginOptionsDefaultLayouts = {
  readonly default: Maybe<Scalars['String']>;
};

type SitePluginPluginOptionsDefaultLayoutsFilterInput = {
  readonly default: Maybe<StringQueryOperatorInput>;
};

type SitePluginPluginOptionsFilterInput = {
  readonly implementation: Maybe<SitePluginPluginOptionsImplementationFilterInput>;
  readonly indentedSyntax: Maybe<BooleanQueryOperatorInput>;
  readonly indentType: Maybe<StringQueryOperatorInput>;
  readonly indentWidth: Maybe<IntQueryOperatorInput>;
  readonly linefeed: Maybe<StringQueryOperatorInput>;
  readonly omitSourceMapUrl: Maybe<BooleanQueryOperatorInput>;
  readonly precision: Maybe<IntQueryOperatorInput>;
  readonly sourceComments: Maybe<BooleanQueryOperatorInput>;
  readonly sourceMapContents: Maybe<BooleanQueryOperatorInput>;
  readonly sourceMapEmbed: Maybe<BooleanQueryOperatorInput>;
  readonly base64Width: Maybe<IntQueryOperatorInput>;
  readonly stripMetadata: Maybe<BooleanQueryOperatorInput>;
  readonly defaultQuality: Maybe<IntQueryOperatorInput>;
  readonly failOnError: Maybe<BooleanQueryOperatorInput>;
  readonly output: Maybe<StringQueryOperatorInput>;
  readonly createLinkInHead: Maybe<BooleanQueryOperatorInput>;
  readonly icon: Maybe<StringQueryOperatorInput>;
  readonly legacy: Maybe<BooleanQueryOperatorInput>;
  readonly theme_color_in_head: Maybe<BooleanQueryOperatorInput>;
  readonly cache_busting_mode: Maybe<StringQueryOperatorInput>;
  readonly crossOrigin: Maybe<StringQueryOperatorInput>;
  readonly include_favicon: Maybe<BooleanQueryOperatorInput>;
  readonly cacheDigest: Maybe<StringQueryOperatorInput>;
  readonly defaultLayouts: Maybe<SitePluginPluginOptionsDefaultLayoutsFilterInput>;
  readonly extensions: Maybe<StringQueryOperatorInput>;
  readonly lessBabel: Maybe<BooleanQueryOperatorInput>;
  readonly mediaTypes: Maybe<StringQueryOperatorInput>;
  readonly name: Maybe<StringQueryOperatorInput>;
  readonly path: Maybe<StringQueryOperatorInput>;
  readonly style: Maybe<BooleanQueryOperatorInput>;
  readonly lessOptions: Maybe<SitePluginPluginOptionsLessOptionsFilterInput>;
  readonly pathCheck: Maybe<BooleanQueryOperatorInput>;
  readonly allExtensions: Maybe<BooleanQueryOperatorInput>;
  readonly isTSX: Maybe<BooleanQueryOperatorInput>;
  readonly jsxPragma: Maybe<StringQueryOperatorInput>;
};

type SitePluginPluginOptionsImplementation = {
  readonly info: Maybe<Scalars['String']>;
};

type SitePluginPluginOptionsImplementationFilterInput = {
  readonly info: Maybe<StringQueryOperatorInput>;
};

type SitePluginPluginOptionsLessOptions = {
  readonly modifyVars: Maybe<SitePluginPluginOptionsLessOptionsModifyVars>;
  readonly javascriptEnabled: Maybe<Scalars['Boolean']>;
};

type SitePluginPluginOptionsLessOptionsFilterInput = {
  readonly modifyVars: Maybe<SitePluginPluginOptionsLessOptionsModifyVarsFilterInput>;
  readonly javascriptEnabled: Maybe<BooleanQueryOperatorInput>;
};

type SitePluginPluginOptionsLessOptionsModifyVars = {
  readonly primary_color: Maybe<Scalars['String']>;
};

type SitePluginPluginOptionsLessOptionsModifyVarsFilterInput = {
  readonly primary_color: Maybe<StringQueryOperatorInput>;
};

type SitePluginSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<SitePluginFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

type SiteSiteMetadata = {
  readonly title: Maybe<Scalars['String']>;
  readonly description: Maybe<Scalars['String']>;
  readonly siteUrl: Maybe<Scalars['String']>;
};

type SiteSiteMetadataFilterInput = {
  readonly title: Maybe<StringQueryOperatorInput>;
  readonly description: Maybe<StringQueryOperatorInput>;
  readonly siteUrl: Maybe<StringQueryOperatorInput>;
};

type SiteSortInput = {
  readonly fields: Maybe<ReadonlyArray<Maybe<SiteFieldsEnum>>>;
  readonly order: Maybe<ReadonlyArray<Maybe<SortOrderEnum>>>;
};

enum SortOrderEnum {
  ASC = 'ASC',
  DESC = 'DESC'
}

type StringQueryOperatorInput = {
  readonly eq: Maybe<Scalars['String']>;
  readonly ne: Maybe<Scalars['String']>;
  readonly in: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly nin: Maybe<ReadonlyArray<Maybe<Scalars['String']>>>;
  readonly regex: Maybe<Scalars['String']>;
  readonly glob: Maybe<Scalars['String']>;
};

type TransformOptions = {
  readonly grayscale: Maybe<Scalars['Boolean']>;
  readonly duotone: Maybe<DuotoneGradient>;
  readonly rotate: Maybe<Scalars['Int']>;
  readonly trim: Maybe<Scalars['Float']>;
  readonly cropFocus: Maybe<ImageCropFocus>;
  readonly fit: Maybe<ImageFit>;
};

type WebPOptions = {
  readonly quality: Maybe<Scalars['Int']>;
};

type PageMetaQueryVariables = Exact<{ [key: string]: never; }>;


type PageMetaQuery = { readonly allMdx: { readonly nodes: ReadonlyArray<(
      Pick<Mdx, 'tableOfContents' | 'slug' | 'timeToRead'>
      & { readonly frontmatter: Maybe<Pick<MdxFrontmatter, 'title'>> }
    )> } };

type NavPagesQueryVariables = Exact<{ [key: string]: never; }>;


type NavPagesQuery = { readonly allMdx: { readonly nodes: ReadonlyArray<(
      Pick<Mdx, 'tableOfContents' | 'slug'>
      & { readonly frontmatter: Maybe<Pick<MdxFrontmatter, 'nav' | 'navOrder' | 'title'>> }
    )> } };

type SchemasQueryVariables = Exact<{ [key: string]: never; }>;


type SchemasQuery = { readonly allSchemasJson: { readonly nodes: ReadonlyArray<(
      Pick<SchemasJson, 'description' | 'title'>
      & { readonly x_taplo_info: Maybe<Pick<SchemasJsonX_taplo_info, 'authors' | 'patterns'>>, readonly parent: Maybe<Pick<File, 'name'>> }
    )> } };

type GatsbyImageSharpFixedFragment = Pick<ImageSharpFixed, 'base64' | 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpFixed_tracedSVGFragment = Pick<ImageSharpFixed, 'tracedSVG' | 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpFixed_withWebpFragment = Pick<ImageSharpFixed, 'base64' | 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpFixed_withWebp_tracedSVGFragment = Pick<ImageSharpFixed, 'tracedSVG' | 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpFixed_noBase64Fragment = Pick<ImageSharpFixed, 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpFixed_withWebp_noBase64Fragment = Pick<ImageSharpFixed, 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpFluidFragment = Pick<ImageSharpFluid, 'base64' | 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpFluidLimitPresentationSizeFragment = { maxHeight: ImageSharpFluid['presentationHeight'], maxWidth: ImageSharpFluid['presentationWidth'] };

type GatsbyImageSharpFluid_tracedSVGFragment = Pick<ImageSharpFluid, 'tracedSVG' | 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpFluid_withWebpFragment = Pick<ImageSharpFluid, 'base64' | 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

type GatsbyImageSharpFluid_withWebp_tracedSVGFragment = Pick<ImageSharpFluid, 'tracedSVG' | 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

type GatsbyImageSharpFluid_noBase64Fragment = Pick<ImageSharpFluid, 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpFluid_withWebp_noBase64Fragment = Pick<ImageSharpFluid, 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

type GatsbyImageSharpResolutionsFragment = Pick<ImageSharpResolutions, 'base64' | 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpResolutions_tracedSVGFragment = Pick<ImageSharpResolutions, 'tracedSVG' | 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpResolutions_withWebpFragment = Pick<ImageSharpResolutions, 'base64' | 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpResolutions_withWebp_tracedSVGFragment = Pick<ImageSharpResolutions, 'tracedSVG' | 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpResolutions_noBase64Fragment = Pick<ImageSharpResolutions, 'width' | 'height' | 'src' | 'srcSet'>;

type GatsbyImageSharpResolutions_withWebp_noBase64Fragment = Pick<ImageSharpResolutions, 'width' | 'height' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp'>;

type GatsbyImageSharpSizesFragment = Pick<ImageSharpSizes, 'base64' | 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpSizes_tracedSVGFragment = Pick<ImageSharpSizes, 'tracedSVG' | 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpSizes_withWebpFragment = Pick<ImageSharpSizes, 'base64' | 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

type GatsbyImageSharpSizes_withWebp_tracedSVGFragment = Pick<ImageSharpSizes, 'tracedSVG' | 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

type GatsbyImageSharpSizes_noBase64Fragment = Pick<ImageSharpSizes, 'aspectRatio' | 'src' | 'srcSet' | 'sizes'>;

type GatsbyImageSharpSizes_withWebp_noBase64Fragment = Pick<ImageSharpSizes, 'aspectRatio' | 'src' | 'srcSet' | 'srcWebp' | 'srcSetWebp' | 'sizes'>;

}