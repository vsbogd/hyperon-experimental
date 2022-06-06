import unittest

from common import MeTTa

class MeTTaTest(unittest.TestCase):

    def test_run_metta(self):
        # REM: this is the initial implementation, which can be
        #      moved to MeTTa class later or changed
        program = '''
            (isa red color)
            (isa green color)
            (isa blue color)
            ;(isa comment color)
            !(match &self (isa $color color) $color)

            (= (f) (+ 2 3))
            !(f)
        '''

        result = MeTTa().run(program)
        self.assertEqual('[[red, green, blue], [5]]', repr(result))

    def test_run_complex_query(self):
        program = '''
            (A B)
            (C B)

            !(match &self (, (A $x) (C $x)) $x)
        '''

        result = MeTTa().run(program)
        self.assertEqual('[[B]]', repr(result))

    def test_list_concatenation(self):
        program = '''
            (= (Concat (Cons $head1 Nil) $list2)
               (Cons $head1 $list2))

            (= (Concat (Cons $head1 (Cons $t1 $t11)) $list2)
               (Cons $head1 (Concat (Cons $t1 $t11) $list2)))

            (= (lst1) (Cons a1 (Cons a2 Nil)))
            (= (lst2) (Cons b1 (Cons b2 Nil)))
            !(Concat (lst1) (lst2))
        '''

        result = MeTTa().run(program)
        self.assertEqual('[[(Cons a1 (Cons a2 (Cons b1 (Cons b2 Nil))))]]', repr(result))
