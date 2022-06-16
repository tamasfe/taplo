export const comment = {
    captures: {
        1: {
            name: 'comment.line.number-sign.toml',
        },
        2: {
            name: 'punctuation.definition.comment.toml',
        },
    },
    comment: 'Comments',
    match: '\\s*((#).*)$',
}

export const commentDirective = {
    captures: {
        1: {
            name: 'meta.preprocessor.toml',
        },
        2: {
            name: 'punctuation.definition.meta.preprocessor.toml',
        },
    },
    comment: 'Comments',
    match: '\\s*((#):.*)$',
}

