# Copyright 2023 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import argparse
import logging
from pathlib import Path
import pdb
import re
import sys
import xml.etree.ElementTree as etree

sys.path.insert(
    0, str(Path(__file__).absolute().parents[1] / "third_party" / "Vulkan-Headers" / "registry")
)  # noqa: E402

from generator import OutputGenerator, GeneratorOptions
from layer_trait_generator import LayerTraitGenerator
from global_simple_intercept_generator import GlobalSimpleInterceptGenerator
from reg import Registry
from vkconventions import VulkanConventions
from spec_tools.conventions import ConventionsBase


def make_re_string(strings, default=None, strings_are_regex=False):
    """Turn a list of strings into a regexp string matching exactly those strings."""
    if strings or default is None:
        if not strings_are_regex:
            strings = (re.escape(s) for s in strings)
        return "^(" + "|".join(strings) + ")$"
    return default


def make_gen_opts(
    conventions: ConventionsBase,
    directory: str,
    feature_pat: str,
    add_extension_pat: str,
    remove_extension_pat: str,
    emit_extension_pat: str,
) -> dict[str, tuple[OutputGenerator, GeneratorOptions]]:
    return {
        "layer_trait.rs": (
            LayerTraitGenerator,
            GeneratorOptions(
                conventions=conventions,
                filename="layer_trait.rs",
                directory=directory,
                genpath=None,
                apiname="vulkan",
                profile=None,
                versions=feature_pat,
                emitversions=feature_pat,
                defaultExtensions="vulkan",
                addExtensions=add_extension_pat,
                removeExtensions=remove_extension_pat,
                emitExtensions=emit_extension_pat,
            ),
        ),
        "global_simple_intercept.rs": (
            GlobalSimpleInterceptGenerator,
            GeneratorOptions(
                conventions=conventions,
                filename="global_simple_intercept.rs",
                directory=directory,
                genpath=None,
                apiname="vulkan",
                profile=None,
                versions=feature_pat,
                emitversions=feature_pat,
                defaultExtensions="vulkan",
                addExtensions=add_extension_pat,
                removeExtensions=remove_extension_pat,
                emitExtensions=emit_extension_pat,
            ),
        ),
    }


def main():
    logging.basicConfig(
        format=(
            "[%(asctime)s %(levelname)s %(module)s] %(funcName)s(%(filename)s:%(lineno)d): "
            "%(message)s"
        ),
        level=logging.INFO,
    )

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-registry",
        action="store",
        required=True,
        help="Use specified registry file instead of vk.xml",
    )
    parser.add_argument("target", metavar="target", help="Specify target")
    parser.add_argument("-dump", action="store_true", help="Enable dump to stderr")
    parser.add_argument("-debug", action="store_true", help="Enable debugging")
    parser.add_argument(
        "-quiet",
        action="store_true",
        default=True,
        help="Suppress script output during normal execution.",
    )
    parser.add_argument(
        "-feature",
        action="append",
        default=[],
        help="Specify a core API feature name or names to add to targets",
    )
    parser.add_argument(
        "-o",
        action="store",
        dest="directory",
        default=".",
        help="Create target and related files in specified directory",
    )
    parser.add_argument(
        "-extension",
        action="append",
        default=[],
        help="Specify an extension or extensions to add to targets",
    )
    parser.add_argument(
        "-remove-extensions",
        action="append",
        default=[],
        help="Specify an extension or extensions to remove from targets",
    )
    parser.add_argument(
        "-emit-extensions",
        action="append",
        default=[],
        help="Specify an extension or extensions to emit in targets",
    )
    args = parser.parse_args()

    args.feature = [name for arg in args.feature for name in arg.split()]
    args.extension = [name for arg in args.extension for name in arg.split()]

    all_features = all_extensions = r".*"

    gen_opts = make_gen_opts(
        conventions=VulkanConventions(),
        directory=args.directory,
        feature_pat=make_re_string(args.feature, all_features),
        add_extension_pat=make_re_string(args.extension, None),
        remove_extension_pat=make_re_string(args.remove_extensions, None),
        emit_extension_pat=make_re_string(args.emit_extensions, all_extensions),
    )
    gen_opt = gen_opts.get(args.target)

    if gen_opt is None:
        logging.fatal("target %s not found", args.target)
        return
    (generator_creator, generator_option) = gen_opt
    generator = generator_creator(err_file=sys.stderr, warn_file=sys.stderr, diag_file=None)

    reg = Registry(generator, generator_option)
    tree = etree.parse(args.registry)

    # Allow the generation of the VK_ANDROID_native_buffer extension.
    for android_ext_elem in tree.findall(
        ".//extension[@name='VK_ANDROID_native_buffer'][@supported]"
    ):
        android_ext_elem.set("supported", "vulkan")

    # Load the XML tree into the registry object
    reg.loadElementTree(tree)

    if args.dump:
        logging.info("* Dumping registry to regdump.txt")
        reg.dumpReg(filehandle=open("regdump.txt", "w", encoding="utf-8"))

    # Finally, use the output generator to create the requested target
    if args.debug:
        pdb.run("reg.apiGen()")
    else:
        reg.apiGen()

    if not args.quiet:
        logging.info("* Generated ", generator_option.filename)


if __name__ == "__main__":
    main()
