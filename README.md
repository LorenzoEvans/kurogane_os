# kurogane_os

### Kurogane is an operating system (well, a kernel), based on Intel's x86_64 architecture, that is capable of being written to a thumb-drive, bare-metal, or ran in a container (this project is specifically written to be booted in QEMU), as instructed by the course laid out here: [Writing An OS In Rust](https://os.phil-opp.com), by [Phil Opperman](https://twitter.com/phil_opp).

#### My interest in following this course include, but are not limited to the following:
- I'm quite passionate about systems (check out my article on this [Embarking Into Systems](https://www.synthsforcompilers.dev/systems/2020/may/systems-post/), writ large, however one of the most common types of systems that I encounter, as a programmer, is the _Operating System_, and I decided that it would be beneficial to learn about the system I'm working on top of, that's responsible for giving my programs access to memory, or making sure my threads don't try and read and write to the same heap object at the same time.
- I wanted to expand my knowledge of the [Rust](https://www.rust-lang.org/) language, while also
taking advantage of the ability to do low-level style programming, with the added advantage of several of the more
tedious aspects of this field being removed, due to the intentions of the language designers, for a language designed for such
tasks, it's still extremely human readable, concise, and the learning curve is made much lower by the high quality documentation.
> I particularly enjoyed learning about [CPU Exceptions](https://roamresearch.com/#/app/0xLEDEV-HQ/page/KZocfr031), and [real mode](https://roamresearch.com/#/app/0xLEDEV-HQ/page/nvPhO-wvo)
 - I wanted to push my knowledge of Computer Science further, and beyond the level of things like algorithms, data structures, or abstract data types- perhaps desiring more of the "_Computer_" and less of the "_Science_", and working on a kernel, has exposed me to a level of deep computational knowledge that I wouldn't have had otherwise, such as what _triple-faults_, do, [what is the relationship between heap and stack allocation](https://roamresearch.com/#/app/0xLEDEV-HQ/page/O6UXD4XXX), and the difference between [ROM](https://roamresearch.com/#/app/0xLEDEV-HQ/page/CDSP3wML6) & [RAM](https://roamresearch.com/#/app/0xLEDEV-HQ/page/w-4N6FTnG).
