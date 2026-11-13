(function () {
  "use strict";

  var figureSelector = "[data-scienceview-figure]";
  var canvasSelector = "[data-scienceview-canvas]";
  var payloadSelector = "script[data-scienceview-payload]";

  function parsePayload(value) {
    if (typeof value !== "string") {
      return value;
    }
    try {
      return JSON.parse(value);
    } catch (error) {
      return null;
    }
  }

  function readView(figure) {
    var value = figure.getAttribute("data-scienceview-view");
    if (!value) {
      return { x: 0, y: 0, scale: 1 };
    }
    try {
      var view = JSON.parse(value);
      return {
        x: Number(view.x) || 0,
        y: Number(view.y) || 0,
        scale: Number(view.scale) || 1
      };
    } catch (error) {
      return { x: 0, y: 0, scale: 1 };
    }
  }

  function writeView(figure, view) {
    figure.setAttribute("data-scienceview-view", JSON.stringify(view));
    var canvas = figure.querySelector(canvasSelector);
    if (canvas) {
      canvas.style.transform =
        "translate(" + view.x + "px," + view.y + "px) scale(" + view.scale + ")";
    }
  }

  function applyDelta(figure, action) {
    if (!action || typeof action !== "object") {
      return;
    }
    var view = readView(figure);
    if (action.type === "pan") {
      view.x += Number(action.dx) || 0;
      view.y += Number(action.dy) || 0;
    } else if (action.type === "zoom") {
      var factor = Number(action.factor) || 1;
      view.scale = Math.min(8, Math.max(0.25, view.scale * factor));
    } else {
      return;
    }
    writeView(figure, view);
    figure.dispatchEvent(
      new CustomEvent("scienceview:delta", { detail: view, bubbles: true })
    );
  }

  function applyPayload(figure, payload) {
    payload = parsePayload(payload);
    if (!payload || typeof payload !== "object") {
      return false;
    }
    var canvas = figure.querySelector(canvasSelector);
    if (!canvas) {
      return false;
    }
    if (typeof payload.markup === "string") {
      canvas.innerHTML = payload.markup;
    }
    figure.setAttribute("data-scienceview-revision", String(Number(payload.revision) || 0));
    figure.setAttribute("data-scienceview-points", String(Number(payload.points) || 0));
    if (payload.clock && payload.clock.reduced_motion) {
      figure.setAttribute("data-scienceview-reduced-motion", "true");
    }
    writeView(figure, readView(figure));
    return true;
  }

  function schedule(figure, payload) {
    payload = parsePayload(payload);
    if (!payload || typeof payload !== "object") {
      return;
    }
    if (payload.clock && payload.clock.reduced_motion) {
      applyPayload(figure, payload);
      return;
    }
    if (typeof window.requestAnimationFrame !== "function") {
      applyPayload(figure, payload);
      return;
    }
    window.requestAnimationFrame(function () {
      applyPayload(figure, payload);
    });
  }

  function observePayload(figure) {
    if (typeof window.MutationObserver !== "function") {
      return;
    }
    var observer = new window.MutationObserver(function (records) {
      records.forEach(function (record) {
        Array.prototype.forEach.call(record.addedNodes, function (node) {
          if (node.nodeType === 1 && node.matches && node.matches(payloadSelector)) {
            schedule(figure, node.textContent || "");
          }
        });
      });
    });
    observer.observe(figure, { childList: true, subtree: true });
  }

  function registerGesture(figure) {
    var drag = null;
    figure.addEventListener("pointerdown", function (event) {
      if (event.button !== 0) {
        return;
      }
      drag = { x: event.clientX, y: event.clientY };
      if (figure.setPointerCapture) {
        figure.setPointerCapture(event.pointerId);
      }
    });
    figure.addEventListener("pointermove", function (event) {
      if (!drag) {
        return;
      }
      var action = {
        type: "pan",
        dx: event.clientX - drag.x,
        dy: event.clientY - drag.y
      };
      drag = { x: event.clientX, y: event.clientY };
      figure.dispatchEvent(
        new CustomEvent("scienceview:action", { detail: action, bubbles: true })
      );
      applyDelta(figure, action);
    });
    figure.addEventListener("pointerup", function () {
      drag = null;
    });
    figure.addEventListener("pointercancel", function () {
      drag = null;
    });
    figure.addEventListener(
      "wheel",
      function (event) {
        event.preventDefault();
        var action = { type: "zoom", factor: event.deltaY < 0 ? 1.1 : 0.9 };
        figure.dispatchEvent(
          new CustomEvent("scienceview:action", { detail: action, bubbles: true })
        );
        applyDelta(figure, action);
      },
      { passive: false }
    );
  }

  function register(figure) {
    if (figure.getAttribute("data-scienceview-bound") === "true") {
      return;
    }
    figure.setAttribute("data-scienceview-bound", "true");
    figure.addEventListener("scienceview:update", function (event) {
      schedule(figure, event.detail);
    });
    observePayload(figure);
    registerGesture(figure);
  }

  window.scienceview = window.scienceview || {};
  window.scienceview.applyPayload = applyPayload;
  window.scienceview.update = function (figure, payload) {
    var element =
      typeof figure === "string" ? document.getElementById(figure) : figure;
    if (!element) {
      return false;
    }
    return applyPayload(element, payload);
  };

  document.querySelectorAll(figureSelector).forEach(register);
}());
