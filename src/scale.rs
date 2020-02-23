use super::{BucketVec, BucketVecConfig};

impl<T, C> scale::Encode for BucketVec<T, C>
where
    T: scale::Encode,
{
    fn encode_to<O: scale::Output>(&self, output: &mut O) {
        output.push(&scale::Compact(self.len() as u64));
        for elem in self {
            output.push(elem);
        }
    }
}

impl<T, C> scale::Decode for BucketVec<T, C>
where
    C: BucketVecConfig,
    T: scale::Decode,
{
    fn decode<I: scale::Input>(input: &mut I) -> Result<Self, scale::Error> {
        let len = <scale::Compact<u64> as scale::Decode>::decode(input)?.0;
        let mut vec = Self::new();
        for _ in 0..len {
            vec.push(<T as scale::Decode>::decode(input)?);
        }
        Ok(vec)
    }
}
