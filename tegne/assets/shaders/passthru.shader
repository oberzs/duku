vert.spv                                                                                                                    00000007600 00000000000 0005344                                                                                                      ustar                                                                                                                                                                                                                                                          #     W                 GLSL.std.450                      main    	      $   &   )   +   -   .   5   :   G   Q        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   out_normal       Constants            model_matrix            albedo_index            sampler_index        object       in_normal     $   out_uv    &   in_uv     )   out_color     +   in_color      -   out_worldspace_position   .   in_worldspace_position    5   out_modelspace_position  	 :   out_screenspace_position      ;   Light     ;       coords    ;      color     ?   WorldObject   ?       world_matrix      ?      light_matrix      ?      lights    ?      camera_position   ?      time      ?      cascade_splits    ?      shadow_index      A   world     G   out_lightspace_position   O   gl_PerVertex      O       gl_Position   O      gl_PointSize      O      gl_ClipDistance   O      gl_CullDistance   Q         T   MaterialObject    T       albedo_tint   T      font_width    T      font_border_tint      T      font_edge     T      font_border_offset    T      font_border_width     T      font_border_edge      T      arg_1     T      arg_2     T   	   arg_3     T   
   arg_4     V   material    G  	          H            H         #       H               H        #   @   H        #   D   G        G           G  $         G  &         G  )         G  +         G  -         G  .          G  5         G  :         H  ;       #       H  ;      #      G  >          H  ?          H  ?       #       H  ?             H  ?         H  ?      #   @   H  ?            H  ?      #   �   H  ?      #      H  ?      #     H  ?      #     H  ?      #     G  ?      G  A   "       G  A   !       G  G         H  O              H  O            H  O            H  O            G  O      H  T       #       H  T      #      H  T      #      H  T      #      H  T      #       H  T      #   (   H  T      #   ,   H  T      #   0   H  T      #   @   H  T   	   #   P   H  T   
   #   `   G  T      G  V   "      G  V   !            !                                        ;     	        
              
                                      	      ;        	   +                  	                             ;             "            #      "   ;  #   $         %      "   ;  %   &         (      
   ;  (   )         *      
   ;  *   +      ;  (   -      ;     .      +     0     �?;  (   5      ;  (   :        ;   
   
     <           +  <   =        >   ;   =    	 ?         >                  @      ?   ;  @   A         B         ;  (   G      +     H      +  <   M        N      M     O   
      N   N      P      O   ;  P   Q        T               "         
   
   
   
      U      T   ;  U   V      6               �     A              =                      "      T           Q  
             O                        Q  
            O                        Q  
            O                        P                 =            �     !          >  	   !   =  "   '   &   >  $   '   =  
   ,   +   >  )   ,   =     /   .   Q     1   /       Q     2   /      Q     3   /      P  
   4   1   2   3   0   >  -   4   A     6         =     7   6   =  
   8   -   �  
   9   7   8   >  5   9   A  B   C   A      =     D   C   =  
   E   5   �  
   F   D   E   >  :   F   A  B   I   A   H   =     J   I   =  
   K   5   �  
   L   J   K   >  G   L   =  
   R   -   A  (   S   Q      >  S   R   �  8                                                                                                                                  frag.spv                                                                                                                    00000006524 00000000000 0005307                                                                                                      ustar                                                                                                                                                                                                                                                          #     M                 GLSL.std.450                     main       -   F   H   I   J   K   L                �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         fragment(        out_color        textures         Constants            model_matrix            albedo_index            sampler_index        object    "   samplers      -   in_uv     1   MaterialObject    1       albedo_tint   1      font_width    1      font_border_tint      1      font_edge     1      font_border_offset    1      font_border_width     1      font_border_edge      1      arg_1     1      arg_2     1   	   arg_3     1   
   arg_4     3   material      ?   Light     ?       coords    ?      color     B   WorldObject   B       world_matrix      B      light_matrix      B      lights    B      camera_position   B      time      B      cascade_splits    B      shadow_index      D   world     F   in_normal     H   in_color      I   in_screenspace_position   J   in_modelspace_position    K   in_worldspace_position    L   in_lightspace_position  G            G     "      G     !       H            H         #       H               H        #   @   H        #   D   G        G  "   "      G  "   !      G  -         H  1       #       H  1      #      H  1      #      H  1      #      H  1      #       H  1      #   (   H  1      #   ,   H  1      #   0   H  1      #   @   H  1   	   #   P   H  1   
   #   `   G  1      G  3   "      G  3   !       H  ?       #       H  ?      #      G  A          H  B          H  B       #       H  B             H  B         H  B      #   @   H  B            H  B      #   �   H  B      #      H  B      #     H  B      #     H  B      #     G  B      G  D   "       G  D   !       G  F          G  H         G  I         G  J         G  K         G  L              !                   	            
      	   ;  
          	                                          +        d                           ;                 	                                      	      ;        	   +                 	                        +                          !           ;  !   "       +     #         &            )        +            ,      +   ;  ,   -        0           1   0      0      +         	   	   	   	      2      1   ;  2   3      +     4          5      0   +     8     �?  ?   	   	   +     @        A   ?   @    	 B         A   0      0         C      B   ;  C   D         E      0   ;  E   F         G      	   ;  G   H      ;  G   I      ;  G   J      ;  G   K      ;  G   L      6               �     9     >      �  8  6               �     A              =           A              =           A     $      #   =     %   $   A  &   '   "   %   =     (   '   V  )   *      (   =  +   .   -   W  	   /   *   .   A  5   6   3   4   =  0   7   6   Q     9   7       Q     :   7      Q     ;   7      P  	   <   9   :   ;   8   �  	   =   /   <   >     =   �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              