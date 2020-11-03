"use strict";
exports.__esModule = true;
exports.generatePagination = exports.sortArticle = exports.filterArticle = exports.compareDate = exports.getDate = void 0;
var dayjs = require("dayjs");
/** 处理日期 */
exports.getDate = function (dateString) {
    var time = dayjs(dateString.trim());
    if (time.isValid()) {
        var year_1 = time.year();
        var month_1 = time.month() + 1;
        var date = time.date();
        var hour_1 = time.hour();
        var minute_1 = time.minute();
        var second_1 = time.second();
        var millisecond = time.millisecond();
        if ((hour_1 === 8 || hour_1 === 0) &&
            minute_1 === 0 &&
            second_1 === 0 &&
            millisecond === 0)
            return [year_1, month_1, date, undefined, undefined, undefined];
        return [year_1, month_1, date, hour_1, minute_1, second_1];
    }
    var pattern = /(?:(\d+)[/-](\d+)[/-](\d+))?\s*(?:(\d+):(\d+)(?::(\d+))?)?/u;
    var _a = pattern.exec(dateString.trim()) || [], year = _a[1], month = _a[2], day = _a[3], hour = _a[4], minute = _a[5], second = _a[6];
    var getNumber = function (a) {
        return typeof a === "undefined" ? undefined : Number(a);
    };
    var getYear = function (yearNumber) {
        return yearNumber && yearNumber < 100 ? yearNumber + 2000 : yearNumber;
    };
    var getSecond = function (secondNumber) {
        return hour && minute && !second ? 0 : secondNumber;
    };
    return [
        getYear(getNumber(year)),
        getNumber(month),
        getNumber(day),
        getNumber(hour),
        getNumber(minute),
        getSecond(getNumber(second)),
    ];
};
/**
 * 日期比较
 * @param dateA 比较的日期A
 * @param dateB 比较的日期B
 */
exports.compareDate = function (dataA, dataB) {
    if (!dataA)
        return 1;
    if (!dataB)
        return -1;
    var compare = function (a, b) {
        if (a.length === 0)
            return 0;
        if (typeof b[0] === "undefined")
            return typeof a[0] === "undefined" || a[0] === 0 ? 0 : -1;
        if (typeof a[0] === "undefined")
            return b[0] === 0 ? 0 : 1;
        if (b[0] - a[0] === 0) {
            a.shift();
            b.shift();
            return compare(a, b);
        }
        return b[0] - a[0];
    };
    return compare(exports.getDate(dataA), exports.getDate(dataB));
};
/**
 * 过滤文章
 *
 * @param pages 页面
 * @param filterFunc 额外的过滤函数
 */
exports.filterArticle = function (pages, filterFunc) {
    return pages.filter(function (page) {
        var _a = page.frontmatter, article = _a.article, blogpage = _a.blogpage, home = _a.home, title = page.title;
        return (typeof title !== "undefined" &&
            blogpage !== true &&
            home !== true &&
            article !== false &&
            (!filterFunc || filterFunc(page.frontmatter)));
    });
};
/** 排序文章 */
exports.sortArticle = function (pages) {
    return pages.slice(0).sort(function (prev, next) {
        var prevSticky = prev.frontmatter.sticky;
        var nextSticky = next.frontmatter.sticky;
        var prevTime = prev.frontmatter.time ||
            prev.frontmatter.date;
        var nextTime = next.frontmatter.time ||
            next.frontmatter.date;
        if (prevSticky && nextSticky)
            return prevSticky === nextSticky
                ? exports.compareDate(prevTime, nextTime)
                : Number(nextSticky) - Number(prevSticky);
        if (prevSticky && !nextSticky)
            return -1;
        if (!prevSticky && nextSticky)
            return 1;
        return exports.compareDate(prevTime, nextTime);
    });
};
exports.generatePagination = function (pages, perPage) {
    if (perPage === void 0) { perPage = 10; }
    var result = [];
    var index = 0;
    while (index < pages.length) {
        var paginationPage = [];
        for (var i = 0; i < perPage; i++)
            if (index < pages.length) {
                paginationPage.push(pages[index]);
                index += 1;
            }
        result.push(paginationPage);
    }
    return result;
};
