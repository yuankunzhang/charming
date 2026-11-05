/*
* Licensed to the Apache Software Foundation (ASF) under one
* or more contributor license agreements.  See the NOTICE file
* distributed with this work for additional information
* regarding copyright ownership.  The ASF licenses this file
* to you under the Apache License, Version 2.0 (the
* "License"); you may not use this file except in compliance
* with the License.  You may obtain a copy of the License at
*
*   http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing,
* software distributed under the License is distributed on an
* "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
* KIND, either express or implied.  See the License for the
* specific language governing permissions and limitations
* under the License.
*/

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? module.exports = factory() :
    typeof define === 'function' && define.amd ? define(factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, global.barRangeCustomSeriesInstaller = factory());
})(this, (function () { 'use strict';

    var renderItem = function (params, api) {
        var x = api.value(0);
        var valueStart = api.value(1);
        var coordStart = api.coord([x, valueStart]);
        var valueEnd = api.value(2);
        var coordEnd = api.coord([x, valueEnd]);
        var bandWidth = api.coord([1, 0])[0] - api.coord([0, 0])[0];
        var barWidthRaw = params.itemPayload.barWidth;
        if (barWidthRaw == null) {
            barWidthRaw = '70%';
        }
        var barWidth = typeof barWidthRaw === 'string' && barWidthRaw.endsWith('%')
            ? (parseFloat(barWidthRaw) / 100) * bandWidth
            : barWidthRaw;
        var borderRadius = params.itemPayload.borderRadius || 0;
        var bar = {
            type: 'rect',
            shape: {
                x: coordStart[0] - barWidth / 2,
                y: coordStart[1],
                width: barWidth,
                height: coordEnd[1] - coordStart[1],
                r: borderRadius,
            },
            style: {
                fill: api.visual('color'),
            },
        };
        var marginRaw = params.itemPayload.margin;
        var margin = marginRaw == null ? 10 : marginRaw;
        var textTop = {
            type: 'text',
            x: coordEnd[0],
            y: coordEnd[1] - margin,
            style: {
                text: valueEnd.toString() + '℃',
                textAlign: 'center',
                textVerticalAlign: 'bottom',
                fill: '#333',
            },
        };
        var textBottom = {
            type: 'text',
            x: coordStart[0],
            y: coordStart[1] + margin,
            style: {
                text: valueStart.toString() + '℃',
                textAlign: 'center',
                textVerticalAlign: 'top',
                fill: '#333',
            },
        };
        return {
            type: 'group',
            children: [bar, textTop, textBottom],
        };
    };
    var index = {
        install: function (registers) {
            registers.registerCustomSeries('barRange', renderItem);
        },
    };

    return index;

}));
