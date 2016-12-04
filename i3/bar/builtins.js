
// ### ARRAYS ### //
Object.defineProperty(Array.prototype, 'toObject', {
  __proto__: null,
  value: function(arr) {
    let obj = {};
    this.forEach((e) => obj[e[0]] = e[1]);
    return obj;
  }
});

Object.defineProperty(Array.prototype, 'peek', {
  __proto__: null,
  value: function(arr) {
    return this.length > 0 ? this[this.length - 1] : null;
  }
});

Object.defineProperty(Array.prototype, 'zip', {
  __proto__: null,
  value: function(arr) {
    return this.map((e, i) => [e, arr[i]]);
  }
});

Object.defineProperty(Array.prototype, 'zipObject', {
  __proto__: null,
  value: function(arr) {
    let pairs = this.zip(arr);

    let obj = {};
    for(let pair of pairs) obj[pair[0]] = pair[1];
    return obj;
  }
});

Object.defineProperty(Array.prototype, 'trimLast', {
  __proto__: null,
  value: function() {
    return this.reverse().slice(1).reverse();
  }
});

Object.defineProperty(Array.prototype, 'trimFirst', {
  __proto__: null,
  value: function() {
    return this.slice(1);
  }
});

// ### OBJECTS ### //
Object.defineProperty(Object.prototype, 'forEach', {
  __proto__: null,
  value: function(fn) {
    for(let key in this) {
      if(this.hasOwnProperty(key))
        fn(key, this[key]);
    }
  }
});

Object.defineProperty(Object.prototype, 'toArray', {
  __proto__: null,
  value: function() {
    let arr = [];
    this.forEach((k, v) => arr.push([k, v]));
    return arr;
  }
});

Object.defineProperty(Object.prototype, 'mapAll', {
  __proto__: null,
  value: function(keyFn, valFn) {
    return this.toArray().map((e) => [keyFn(e[0], e[1], this), valFn(e[0], e[1], this)]).toObject();
  }
});

Object.defineProperty(Object.prototype, 'mapKeys', {
  __proto__: null,
  value: function(keyFn) {
    return this.mapAll((k, v, obj) => keyFn(k, v, obj), (v) => v);
  }
});

Object.defineProperty(Object.prototype, 'map', {
  __proto__: null,
  value: function(valFn) {
    return this.mapAll((k, v) => k, (k, v, obj) => valFn(obj, v));
  }
});
