package dev.kreuzberg.htmltomarkdown;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Allocates Panama FFM upcall stubs for an IHtmlVisitor implementation, assembles the C vtable in native memory, and
 * provides static registerHtmlVisitor/unregisterHtmlVisitor helpers.
 */
public final class HtmlVisitorBridge implements AutoCloseable {

    private static final Linker LINKER = Linker.nativeLinker();
    private static final MethodHandles.Lookup LOOKUP = MethodHandles.lookup();
    private static final ObjectMapper JSON = new ObjectMapper();

    /** Live registry — keeps Arenas and upcall stubs alive past the register call. */
    private static final ConcurrentHashMap<String, HtmlVisitorBridge> HTML_VISITOR_BRIDGES = new ConcurrentHashMap<>();

    // C vtable: 41 fields (0 plugin methods + 40 trait methods + free_user_data)
    private static final long VTABLE_SIZE = (long) ValueLayout.ADDRESS.byteSize() * 41L;

    private final Arena arena;
    private final MemorySegment vtable;
    private final IHtmlVisitor impl;

    HtmlVisitorBridge(final IHtmlVisitor impl) {
        this.impl = impl;
        this.arena = Arena.ofShared();
        this.vtable = arena.allocate(VTABLE_SIZE);

        try {
            long offset = 0L;

            var stubVisitElementStart = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitElementStart",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitElementStart);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitElementEnd = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitElementEnd",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitElementEnd);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitText = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitText",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitText);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitLink = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitLink",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitLink);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitImage = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitImage",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitImage);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitHeading = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitHeading",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, int.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitHeading);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitCodeBlock = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitCodeBlock",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitCodeBlock);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitCodeInline = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitCodeInline",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitCodeInline);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitListItem = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitListItem",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, boolean.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitListItem);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitListStart = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitListStart",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, boolean.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitListStart);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitListEnd = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitListEnd",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, boolean.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitListEnd);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitTableStart = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitTableStart",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitTableStart);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitTableRow = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitTableRow",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, boolean.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitTableRow);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitTableEnd = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitTableEnd",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitTableEnd);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitBlockquote = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitBlockquote",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, long.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.JAVA_LONG, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitBlockquote);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitStrong = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitStrong",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitStrong);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitEmphasis = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitEmphasis",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitEmphasis);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitStrikethrough = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitStrikethrough",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitStrikethrough);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitUnderline = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitUnderline",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitUnderline);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitSubscript = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitSubscript",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitSubscript);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitSuperscript = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitSuperscript",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitSuperscript);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitMark = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitMark",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitMark);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitLineBreak = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitLineBreak",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitLineBreak);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitHorizontalRule = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitHorizontalRule",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitHorizontalRule);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitCustomElement = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitCustomElement",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitCustomElement);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitDefinitionListStart = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitDefinitionListStart",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitDefinitionListStart);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitDefinitionTerm = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitDefinitionTerm",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitDefinitionTerm);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitDefinitionDescription = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitDefinitionDescription",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitDefinitionDescription);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitDefinitionListEnd = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitDefinitionListEnd",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitDefinitionListEnd);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitForm = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitForm",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitForm);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitInput = LINKER.upcallStub(LOOKUP.bind(this, "handleVisitInput",
                    MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class,
                            MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitInput);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitButton = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitButton",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitButton);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitAudio = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitAudio",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitAudio);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitVideo = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitVideo",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitVideo);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitIframe = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitIframe",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitIframe);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitDetails = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitDetails",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, boolean.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitDetails);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitSummary = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitSummary",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitSummary);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitFigureStart = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitFigureStart",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitFigureStart);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitFigcaption = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitFigcaption",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitFigcaption);
            offset += ValueLayout.ADDRESS.byteSize();

            var stubVisitFigureEnd = LINKER.upcallStub(
                    LOOKUP.bind(this, "handleVisitFigureEnd",
                            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class,
                                    MemorySegment.class, MemorySegment.class, MemorySegment.class)),
                    FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS,
                            ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
                    arena);
            vtable.set(ValueLayout.ADDRESS, offset, stubVisitFigureEnd);
            offset += ValueLayout.ADDRESS.byteSize();

            vtable.set(ValueLayout.ADDRESS, offset, MemorySegment.NULL);

        } catch (ReflectiveOperationException e) {
            arena.close();
            throw new RuntimeException("Failed to create trait bridge stubs", e);
        }
    }

    MemorySegment vtableSegment() {
        return vtable;
    }

    private int handleVisitElementStart(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_element_start(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitElementEnd(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _output_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _output = _output_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_element_end(_ctx, _output);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitText(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_text(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitLink(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _href_in,
            MemorySegment _text_in, MemorySegment _title_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _href = _href_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _title = _title_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_link(_ctx, _href, _text, _title);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitImage(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _src_in,
            MemorySegment _alt_in, MemorySegment _title_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _src = _src_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _alt = _alt_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _title = _title_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_image(_ctx, _src, _alt, _title);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitHeading(MemorySegment userData, MemorySegment _ctx_in, int _level, MemorySegment _text_in,
            MemorySegment _id_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _id = _id_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_heading(_ctx, _level, _text, _id);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitCodeBlock(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _lang_in,
            MemorySegment _code_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _lang = _lang_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _code = _code_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_code_block(_ctx, _lang, _code);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitCodeInline(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _code_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _code = _code_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_code_inline(_ctx, _code);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitListItem(MemorySegment userData, MemorySegment _ctx_in, boolean _ordered,
            MemorySegment _marker_in, MemorySegment _text_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _marker = _marker_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_list_item(_ctx, _ordered, _marker, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitListStart(MemorySegment userData, MemorySegment _ctx_in, boolean _ordered,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_list_start(_ctx, _ordered);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitListEnd(MemorySegment userData, MemorySegment _ctx_in, boolean _ordered,
            MemorySegment _output_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _output = _output_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_list_end(_ctx, _ordered, _output);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitTableStart(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_table_start(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitTableRow(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _cells_in,
            boolean _is_header, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _cells_json = _cells_in.reinterpret(Long.MAX_VALUE).getString(0);
            List<String> _cells = JSON.readValue(_cells_json,
                    new com.fasterxml.jackson.core.type.TypeReference<List<String>>() {
                    });
            VisitResult result = impl.visit_table_row(_ctx, _cells, _is_header);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitTableEnd(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _output_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _output = _output_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_table_end(_ctx, _output);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitBlockquote(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _content_in,
            long _depth, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _content = _content_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_blockquote(_ctx, _content, _depth);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitStrong(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_strong(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitEmphasis(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_emphasis(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitStrikethrough(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_strikethrough(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitUnderline(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_underline(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitSubscript(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_subscript(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitSuperscript(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_superscript(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitMark(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_mark(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitLineBreak(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_line_break(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitHorizontalRule(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_horizontal_rule(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitCustomElement(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _tag_name_in,
            MemorySegment _html_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _tag_name = _tag_name_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _html = _html_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_custom_element(_ctx, _tag_name, _html);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitDefinitionListStart(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_definition_list_start(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitDefinitionTerm(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_definition_term(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitDefinitionDescription(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_definition_description(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitDefinitionListEnd(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _output_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _output = _output_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_definition_list_end(_ctx, _output);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitForm(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _action_in,
            MemorySegment _method_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _action = _action_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _method = _method_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_form(_ctx, _action, _method);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitInput(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _input_type_in,
            MemorySegment _name_in, MemorySegment _value_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _input_type = _input_type_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _name = _name_in.reinterpret(Long.MAX_VALUE).getString(0);
            String _value = _value_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_input(_ctx, _input_type, _name, _value);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitButton(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_button(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitAudio(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _src_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _src = _src_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_audio(_ctx, _src);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitVideo(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _src_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _src = _src_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_video(_ctx, _src);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitIframe(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _src_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _src = _src_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_iframe(_ctx, _src);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitDetails(MemorySegment userData, MemorySegment _ctx_in, boolean _open,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_details(_ctx, _open);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitSummary(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_summary(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitFigureStart(MemorySegment userData, MemorySegment _ctx_in, MemorySegment outResult,
            MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            VisitResult result = impl.visit_figure_start(_ctx);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitFigcaption(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _text_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _text = _text_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_figcaption(_ctx, _text);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleVisitFigureEnd(MemorySegment userData, MemorySegment _ctx_in, MemorySegment _output_in,
            MemorySegment outResult, MemorySegment outError) {
        try {
            String _ctx_json = _ctx_in.reinterpret(Long.MAX_VALUE).getString(0);
            NodeContext _ctx = JSON.readValue(_ctx_json, NodeContext.class);
            String _output = _output_in.reinterpret(Long.MAX_VALUE).getString(0);
            VisitResult result = impl.visit_figure_end(_ctx, _output);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private void writeError(MemorySegment outError, Throwable e) {
        try {
            outError.set(ValueLayout.ADDRESS, 0,
                    arena.allocateFrom(e.getClass().getSimpleName() + ": " + e.getMessage()));
        } catch (Throwable ignored) {
            /* swallow */ }
    }

    @Override
    public void close() {
        arena.close();
    }

    /** Register a HtmlVisitor implementation via Panama FFM upcall stubs. */
    public static void registerHtmlVisitor(final IHtmlVisitor impl, String name) throws Exception {
        var bridge = new HtmlVisitorBridge(impl);
        try {
            try (var nameArena = Arena.ofConfined()) {
                var nameCs = nameArena.allocateFrom(name);
                MemorySegment outErr = nameArena.allocate(ValueLayout.ADDRESS);
                int rc = (int) NativeLib.HTM_REGISTER_HTML_VISITOR.invoke(nameCs, bridge.vtableSegment(),
                        MemorySegment.NULL, outErr);
                if (rc != 0) {
                    MemorySegment errPtr = outErr.get(ValueLayout.ADDRESS, 0);
                    String msg = errPtr.equals(MemorySegment.NULL)
                            ? "registration failed (rc=" + rc + ")"
                            : errPtr.reinterpret(Long.MAX_VALUE).getString(0);
                    throw new RuntimeException("registerHtmlVisitor: " + msg);
                }
            }
        } catch (Throwable t) {
            bridge.close();
            if (t instanceof Exception e) {
                throw e;
            } else {
                throw new RuntimeException("Unexpected error during registration", t);
            }
        }
        HTML_VISITOR_BRIDGES.put(name, bridge);
    }

    /** Unregister a HtmlVisitor implementation by name. */
    public static void unregisterHtmlVisitor(String name) throws Exception {
        try {
            try (var nameArena = Arena.ofConfined()) {
                var nameCs = nameArena.allocateFrom(name);
                MemorySegment outErr = nameArena.allocate(ValueLayout.ADDRESS);
                int rc = (int) NativeLib.HTM_UNREGISTER_HTML_VISITOR.invoke(nameCs, outErr);
                if (rc != 0) {
                    MemorySegment errPtr = outErr.get(ValueLayout.ADDRESS, 0);
                    String msg = errPtr.equals(MemorySegment.NULL)
                            ? "unregistration failed (rc=" + rc + ")"
                            : errPtr.reinterpret(Long.MAX_VALUE).getString(0);
                    throw new RuntimeException("unregisterHtmlVisitor: " + msg);
                }
            }
        } catch (Throwable t) {
            if (t instanceof Exception e) {
                throw e;
            } else {
                throw new RuntimeException("Unexpected error during unregistration", t);
            }
        }
        HtmlVisitorBridge old = HTML_VISITOR_BRIDGES.remove(name);
        if (old != null) {
            old.close();
        }
    }
}
