// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction.html">はじめに</a></span></li><li class="chapter-item expanded "><li class="part-title">00 Introduction</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="00-introduction/00-1-how-to-use.html">この教材の使い方</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>cargo test で学ぶ</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>診断問題</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>「Rustらしさ」とは</span></span></li><li class="chapter-item expanded "><li class="part-title">01〜03 基礎の設計</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="01-ownership/index.html">01 Ownership &amp; Borrowing</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="01-ownership/01-1-move.html">01-1 moveは何を守っているのか</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="01-ownership/01-2-borrow-errors.html">01-2 借用のエラーを読む</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="01-ownership/01-3-ownership-design.html">01-3 所有権で設計する</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="01-ownership/01-4-compare-signatures.html">01-4 シグネチャを比較する</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="02-type-design/index.html">02 Type Design</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="02-type-design/02-1-invalid-states.html">02-1 boolと文字列が表現してしまう不正な状態</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="02-type-design/02-2-newtype.html">02-2 newtype pattern</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="02-type-design/02-3-parse-dont-validate.html">02-3 構築時検証</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="02-type-design/02-4-visibility.html">02-4 フィールドの公開範囲とAPI</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="03-enum-state-machine/index.html">03 Enum &amp; State Machine</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="03-enum-state-machine/03-1-enum-states.html">03-1 enumで状態を表す</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="03-enum-state-machine/03-2-transitions.html">03-2 状態遷移をenumで設計する</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="03-enum-state-machine/03-3-typestate.html">03-3 typestate pattern</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="03-enum-state-machine/03-4-non-exhaustive.html">03-4 #[non_exhaustive] と将来の拡張</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">04〜07 抽象化の設計</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/index.html">04 Traits</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/04-1-trait-purpose.html">04-1 traitは「共通化」のためではない</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/04-2-generic-vs-dyn.html">04-2 generic vs trait object</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/04-3-associated-type.html">04-3 associated type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/04-4-standard-traits.html">04-4 標準trait実装の設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="04-traits/04-5-unnecessary-traits.html">04-5 不要なtraitを見抜く</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="05-generics/index.html">05 Generics</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="05-generics/05-1-generic-benefit.html">05-1 genericにするメリットはあるか</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="05-generics/05-2-trait-bounds.html">05-2 trait boundsの設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="05-generics/05-3-impl-trait.html">05-3 impl Trait</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="05-generics/05-4-gat-basics.html">05-4 GATの基本用途</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="06-error-handling/index.html">06 Error Handling</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="06-error-handling/06-1-result-design.html">06-1 Result の型設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="06-error-handling/06-2-custom-error-types.html">06-2 独自Error型</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="06-error-handling/06-3-error-propagation.html">06-3 error propagation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="06-error-handling/06-4-library-vs-application.html">06-4 libraryとapplicationのerror設計</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="07-iterator/index.html">07 Iterator</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="07-iterator/07-1-adapters-vs-for.html">07-1 adapterの組み合わせ</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="07-iterator/07-2-custom-iterator.html">07-2 Iterator trait</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="07-iterator/07-3-laziness-and-allocation.html">07-3 遅延評価とallocation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="07-iterator/07-4-borrowing-iterators.html">07-4 借用するiteratorを返す</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">08〜11 発展</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="08-lifetimes/index.html">08 Lifetimes</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="08-lifetimes/08-1-what-annotations-say.html">08-1 lifetime annotationは何を主張しているのか</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="08-lifetimes/08-2-structs-with-references.html">08-2 構造体が参照を持つ設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="08-lifetimes/08-3-elision-and-static.html">08-3 elisionと &#39;static</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="08-lifetimes/08-4-hrtb.html">08-4 HRTB入門</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="09-advanced-type-system/index.html">09 Advanced Type System</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="09-advanced-type-system/09-1-phantom-data.html">09-1 PhantomData</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="09-advanced-type-system/09-2-variance.html">09-2 variance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="09-advanced-type-system/09-3-type-level-constraints.html">09-3 型レベルでの制約表現</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="09-advanced-type-system/09-4-zero-cost.html">09-4 zero-cost abstraction</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="10-concurrency/index.html">10 Concurrency</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="10-concurrency/10-1-send-sync.html">10-1 Send / Sync</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="10-concurrency/10-2-arc-mutex.html">10-2 共有と可変性</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="10-concurrency/10-3-interior-mutability.html">10-3 interior mutability</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="10-concurrency/10-4-message-passing.html">10-4 メッセージパッシング</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="11-async/index.html">11 Async Rust</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="11-async/11-1-what-is-a-future.html">11-1 Future とは何か</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="11-async/11-2-pin.html">11-2 Pin / Unpin</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="11-async/11-3-async-and-ownership.html">11-3 asyncと所有権</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="11-async/11-4-async-vs-threads.html">11-4 asyncを使うべきか</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">12〜16 構造と品質</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="12-architecture/index.html">12 Module &amp; Architecture</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="12-architecture/12-1-module-boundaries.html">12-1 module分割の基準</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="12-architecture/12-2-lib-and-bin.html">12-2 lib / bin の分離</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="12-architecture/12-3-workspaces.html">12-3 workspaceの設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="12-architecture/12-4-dependencies-and-visibility.html">12-4 依存の方向と公開範囲</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="13-testing/index.html">13 Testing</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="13-testing/13-1-testable-design.html">13-1 テスト可能な設計</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="13-testing/13-2-kinds-of-tests.html">13-2 unit / integration / doctest</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="13-testing/13-3-testing-failures.html">13-3 失敗ケースのテスト</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="14-macros/index.html">14 Macros</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="14-macros/14-1-when-macros.html">14-1 マクロが必要な瞬間</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="14-macros/14-2-macro-rules.html">14-2 declarative macro</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="14-macros/14-3-proc-macros.html">14-3 procedural macro 入門</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="15-unsafe/index.html">15 Unsafe Rust</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="15-unsafe/15-1-what-unsafe-means.html">15-1 unsafe は何を宣言しているのか</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="15-unsafe/15-2-safe-abstraction.html">15-2 safe abstraction を作る</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="15-unsafe/15-3-ffi.html">15-3 FFI</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="16-library-design/index.html">16 Library Design</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="16-library-design/16-1-api-review.html">16-1 公開APIのレビュー</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="16-library-design/16-2-semver.html">16-2 semver と拡張性</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="16-library-design/16-3-documentation.html">16-3 ドキュメントと例</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">総合</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>17 Practical Projects</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>18 Final Project</span></span></li><li class="chapter-item expanded "><li class="part-title">Appendix</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>A. Compiler Error 読解集</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>B. AIレビュー用プロンプト集</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span>C. 用語集</span></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            // Check both with and without the '.html' suffix to be robust against pretty URLs
            if (link.href.replace(/\.html$/, '') === current_page.replace(/\.html$/, '')
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

